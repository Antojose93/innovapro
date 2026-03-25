use std::collections::{HashMap, HashSet, VecDeque};

use crate::domain::master_data::{
    ConversionResult, ConvertUnitsInput, CreateFinishedProductInput, CreateMaterialInput,
    CreateRecipeInput, CreateUnitConversionInput, CreateUnitInput, MasterDataSnapshot,
    MaterialCostHistoryEntry, RecipeItemPayload, RecordMaterialCostInput,
    UpdateFinishedProductInput, UpdateMaterialInput, UpdateRecipeInput,
    UpdateUnitConversionInput, UpdateUnitInput,
};
use crate::errors::{AppError, AppResult};
use crate::repositories::master_data::MasterDataRepository;
use crate::state::AppState;

pub struct MasterDataService<'a> {
    repository: MasterDataRepository<'a>,
}

impl<'a> MasterDataService<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self {
            repository: MasterDataRepository::new(&state.pool),
        }
    }

    pub async fn get_snapshot(&self) -> AppResult<MasterDataSnapshot> {
        let units = self.repository.list_units().await?;
        let materials = self.repository.list_materials().await?;
        let products = self.repository.list_products().await?;
        let recipes = self.repository.list_recipes().await?;
        let conversions = self.repository.list_conversions().await?;

        Ok(MasterDataSnapshot {
            units,
            materials,
            products,
            recipes,
            conversions,
        })
    }

    pub async fn create_unit(&self, mut input: CreateUnitInput) -> AppResult<MasterDataSnapshot> {
        input.code = input.code.trim().to_uppercase();
        input.name = input.name.trim().to_string();
        input.symbol = input.symbol.trim().to_string();

        if input.code.is_empty() || input.name.is_empty() || input.symbol.is_empty() {
            return Err(AppError::Validation(
                "Todos los campos de la unidad son obligatorios".into(),
            ));
        }

        self.repository
            .create_unit(&input)
            .await
            .map_err(map_unit_write_error)?;
        self.get_snapshot().await
    }

    pub async fn update_unit(&self, mut input: UpdateUnitInput) -> AppResult<MasterDataSnapshot> {
        input.code = input.code.trim().to_uppercase();
        input.name = input.name.trim().to_string();
        input.symbol = input.symbol.trim().to_string();

        if !self.repository.unit_exists(input.id).await? {
            return Err(AppError::Validation("La unidad no existe".into()));
        }

        if input.code.is_empty() || input.name.is_empty() || input.symbol.is_empty() {
            return Err(AppError::Validation(
                "Todos los campos de la unidad son obligatorios".into(),
            ));
        }

        self.repository
            .update_unit(&input)
            .await
            .map_err(map_unit_write_error)?;
        self.get_snapshot().await
    }

    pub async fn delete_unit(&self, unit_id: i64) -> AppResult<MasterDataSnapshot> {
        if !self.repository.unit_exists(unit_id).await? {
            return Err(AppError::Validation("La unidad no existe".into()));
        }

        self.repository
            .delete_unit(unit_id)
            .await
            .map_err(map_unit_write_error)?;
        self.get_snapshot().await
    }

    pub async fn create_material(
        &self,
        mut input: CreateMaterialInput,
    ) -> AppResult<MasterDataSnapshot> {
        input.code = input.code.trim().to_uppercase();
        input.name = input.name.trim().to_string();

        if input.code.is_empty() || input.name.is_empty() {
            return Err(AppError::Validation(
                "El codigo y el nombre de la materia prima son obligatorios".into(),
            ));
        }

        if !self.repository.unit_exists(input.stock_unit_id).await?
            || !self.repository.unit_exists(input.purchase_unit_id).await?
        {
            return Err(AppError::Validation(
                "Las unidades seleccionadas para la materia prima no existen".into(),
            ));
        }

        if let Some(initial_cost) = input.initial_cost {
            if initial_cost < 0.0 {
                return Err(AppError::Validation(
                    "El costo inicial no puede ser negativo".into(),
                ));
            }

            if let Some(initial_cost_unit_id) = input.initial_cost_unit_id {
                if !self.repository.unit_exists(initial_cost_unit_id).await? {
                    return Err(AppError::Validation(
                        "La unidad del costo inicial no existe".into(),
                    ));
                }
            }
        }

        self.repository
            .create_material(&input, input.initial_cost_unit_id)
            .await
            .map_err(map_material_write_error)?;

        self.get_snapshot().await
    }

    pub async fn record_material_cost(
        &self,
        input: RecordMaterialCostInput,
    ) -> AppResult<Vec<MaterialCostHistoryEntry>> {
        if input.unit_cost < 0.0 {
            return Err(AppError::Validation(
                "El costo registrado no puede ser negativo".into(),
            ));
        }

        if !self.repository.material_exists(input.material_id).await? {
            return Err(AppError::Validation("La materia prima no existe".into()));
        }

        if !self.repository.unit_exists(input.unit_id).await? {
            return Err(AppError::Validation("La unidad del costo no existe".into()));
        }

        self.repository.record_material_cost(&input).await?;
        self.repository.list_material_cost_history(input.material_id).await
    }

    pub async fn update_material(
        &self,
        mut input: UpdateMaterialInput,
    ) -> AppResult<MasterDataSnapshot> {
        input.code = input.code.trim().to_uppercase();
        input.name = input.name.trim().to_string();

        if !self.repository.material_exists(input.id).await? {
            return Err(AppError::Validation("La materia prima no existe".into()));
        }

        if input.code.is_empty() || input.name.is_empty() {
            return Err(AppError::Validation(
                "El codigo y el nombre de la materia prima son obligatorios".into(),
            ));
        }

        if !self.repository.unit_exists(input.stock_unit_id).await?
            || !self.repository.unit_exists(input.purchase_unit_id).await?
        {
            return Err(AppError::Validation(
                "Las unidades seleccionadas para la materia prima no existen".into(),
            ));
        }

        self.repository
            .update_material(&input)
            .await
            .map_err(map_material_write_error)?;
        self.get_snapshot().await
    }

    pub async fn delete_material(&self, material_id: i64) -> AppResult<MasterDataSnapshot> {
        if !self.repository.material_exists(material_id).await? {
            return Err(AppError::Validation("La materia prima no existe".into()));
        }

        self.repository.delete_material(material_id).await?;
        self.get_snapshot().await
    }

    pub async fn list_material_cost_history(
        &self,
        material_id: i64,
    ) -> AppResult<Vec<MaterialCostHistoryEntry>> {
        if !self.repository.material_exists(material_id).await? {
            return Err(AppError::Validation("La materia prima no existe".into()));
        }

        self.repository.list_material_cost_history(material_id).await
    }

    pub async fn create_finished_product(
        &self,
        mut input: CreateFinishedProductInput,
    ) -> AppResult<MasterDataSnapshot> {
        normalize_product_input(&mut input);

        validate_product_input(&self.repository, &input, false).await?;

        self.repository
            .create_finished_product(&input)
            .await
            .map_err(map_product_write_error)?;
        self.get_snapshot().await
    }

    pub async fn update_finished_product(
        &self,
        mut input: UpdateFinishedProductInput,
    ) -> AppResult<MasterDataSnapshot> {
        normalize_product_update_input(&mut input);

        if !self.repository.product_exists(input.id).await? {
            return Err(AppError::Validation("El producto no existe".into()));
        }

        validate_product_update_input(&self.repository, &input).await?;

        self.repository
            .update_finished_product(&input)
            .await
            .map_err(map_product_write_error)?;
        self.get_snapshot().await
    }

    pub async fn delete_finished_product(&self, product_id: i64) -> AppResult<MasterDataSnapshot> {
        if !self.repository.product_exists(product_id).await? {
            return Err(AppError::Validation("El producto no existe".into()));
        }

        self.repository.delete_finished_product(product_id).await?;
        self.get_snapshot().await
    }

    pub async fn create_recipe(&self, mut input: CreateRecipeInput) -> AppResult<MasterDataSnapshot> {
        normalize_recipe_input(&mut input);
        validate_recipe_input(&self.repository, &input, false).await?;

        self.repository
            .create_recipe(&input)
            .await
            .map_err(map_recipe_write_error)?;
        self.get_snapshot().await
    }

    pub async fn update_recipe(&self, mut input: UpdateRecipeInput) -> AppResult<MasterDataSnapshot> {
        normalize_recipe_update_input(&mut input);

        if !self.repository.recipe_exists(input.id).await? {
            return Err(AppError::Validation("La receta no existe".into()));
        }

        validate_recipe_update_input(&self.repository, &input).await?;

        self.repository
            .update_recipe(&input)
            .await
            .map_err(map_recipe_write_error)?;
        self.get_snapshot().await
    }

    pub async fn delete_recipe(&self, recipe_id: i64) -> AppResult<MasterDataSnapshot> {
        if !self.repository.recipe_exists(recipe_id).await? {
            return Err(AppError::Validation("La receta no existe".into()));
        }

        self.repository
            .delete_recipe(recipe_id)
            .await
            .map_err(map_recipe_write_error)?;
        self.get_snapshot().await
    }

    pub async fn create_unit_conversion(
        &self,
        input: CreateUnitConversionInput,
    ) -> AppResult<MasterDataSnapshot> {
        validate_conversion_input(&self.repository, input.from_unit_id, input.to_unit_id, input.factor).await?;

        self.repository.create_unit_conversion(&input).await?;
        self.get_snapshot().await
    }

    pub async fn update_unit_conversion(
        &self,
        input: UpdateUnitConversionInput,
    ) -> AppResult<MasterDataSnapshot> {
        if !self.repository.conversion_exists(input.id).await? {
            return Err(AppError::Validation("La conversion no existe".into()));
        }

        validate_conversion_input(&self.repository, input.from_unit_id, input.to_unit_id, input.factor).await?;

        self.repository
            .update_unit_conversion(&input)
            .await
            .map_err(map_conversion_write_error)?;
        self.get_snapshot().await
    }

    pub async fn delete_unit_conversion(&self, conversion_id: i64) -> AppResult<MasterDataSnapshot> {
        if !self.repository.conversion_exists(conversion_id).await? {
            return Err(AppError::Validation("La conversion no existe".into()));
        }

        self.repository.delete_unit_conversion(conversion_id).await?;
        self.get_snapshot().await
    }

    pub async fn convert_units(&self, input: ConvertUnitsInput) -> AppResult<ConversionResult> {
        if input.quantity < 0.0 {
            return Err(AppError::Validation(
                "La cantidad a convertir no puede ser negativa".into(),
            ));
        }

        if input.from_unit_id == input.to_unit_id {
            return Ok(ConversionResult {
                factor: 1.0,
                converted_quantity: input.quantity,
            });
        }

        let conversions = self.repository.list_conversions().await?;
        let factor = resolve_conversion_factor(
            input.from_unit_id,
            input.to_unit_id,
            conversions
                .into_iter()
                .map(|conversion| {
                    (
                        conversion.from_unit_id,
                        conversion.to_unit_id,
                        conversion.factor,
                    )
                })
                .collect(),
        )?;

        Ok(ConversionResult {
            factor,
            converted_quantity: input.quantity * factor,
        })
    }
}

fn normalize_product_input(input: &mut CreateFinishedProductInput) {
    input.code = input.code.trim().to_uppercase();
    input.name = input.name.trim().to_string();
    input.category = input.category.trim().to_string();
    input.status = input.status.trim().to_lowercase();
    input.internal_code = normalize_optional_text(input.internal_code.take());
    input.commercial_code = normalize_optional_text(input.commercial_code.take());
}

fn normalize_product_update_input(input: &mut UpdateFinishedProductInput) {
    input.code = input.code.trim().to_uppercase();
    input.name = input.name.trim().to_string();
    input.category = input.category.trim().to_string();
    input.status = input.status.trim().to_lowercase();
    input.internal_code = normalize_optional_text(input.internal_code.take());
    input.commercial_code = normalize_optional_text(input.commercial_code.take());
}

async fn validate_product_input(
    repository: &MasterDataRepository<'_>,
    input: &CreateFinishedProductInput,
    _is_update: bool,
) -> AppResult<()> {
    if input.code.is_empty() || input.name.is_empty() {
        return Err(AppError::Validation(
            "El codigo y el nombre del producto son obligatorios".into(),
        ));
    }

    if input.category.is_empty() {
        return Err(AppError::Validation(
            "La categoria del producto es obligatoria".into(),
        ));
    }

    if !repository.unit_exists(input.unit_id).await? {
        return Err(AppError::Validation("La unidad del producto no existe".into()));
    }

    if let Some(recipe_id) = input.recipe_id {
        if !repository.recipe_exists(recipe_id).await? {
            return Err(AppError::Validation("La receta seleccionada no existe".into()));
        }
    }

    validate_product_status(&input.status)
}

async fn validate_product_update_input(
    repository: &MasterDataRepository<'_>,
    input: &UpdateFinishedProductInput,
) -> AppResult<()> {
    if input.code.is_empty() || input.name.is_empty() {
        return Err(AppError::Validation(
            "El codigo y el nombre del producto son obligatorios".into(),
        ));
    }

    if input.category.is_empty() {
        return Err(AppError::Validation(
            "La categoria del producto es obligatoria".into(),
        ));
    }

    if !repository.unit_exists(input.unit_id).await? {
        return Err(AppError::Validation("La unidad del producto no existe".into()));
    }

    if let Some(recipe_id) = input.recipe_id {
        if !repository.recipe_exists(recipe_id).await? {
            return Err(AppError::Validation("La receta seleccionada no existe".into()));
        }
    }

    validate_product_status(&input.status)
}

fn normalize_recipe_input(input: &mut CreateRecipeInput) {
    input.code = input.code.trim().to_uppercase();
    input.name = input.name.trim().to_string();
    input.status = input.status.trim().to_lowercase();
    input.version = input.version.trim().to_string();
    input.notes = normalize_optional_text(input.notes.take());
}

fn normalize_recipe_update_input(input: &mut UpdateRecipeInput) {
    input.code = input.code.trim().to_uppercase();
    input.name = input.name.trim().to_string();
    input.status = input.status.trim().to_lowercase();
    input.version = input.version.trim().to_string();
    input.notes = normalize_optional_text(input.notes.take());
}

async fn validate_recipe_input(
    repository: &MasterDataRepository<'_>,
    input: &CreateRecipeInput,
    _is_update: bool,
) -> AppResult<()> {
    if input.code.is_empty() || input.name.is_empty() {
        return Err(AppError::Validation(
            "El codigo y el nombre de la receta son obligatorios".into(),
        ));
    }

    if input.version.is_empty() {
        return Err(AppError::Validation(
            "La version de la receta es obligatoria".into(),
        ));
    }

    if input.output_quantity <= 0.0 {
        return Err(AppError::Validation(
            "El rendimiento de la receta debe ser mayor que cero".into(),
        ));
    }

    if let Some(product_id) = input.product_id {
        if !repository.product_exists(product_id).await? {
            return Err(AppError::Validation(
                "El producto asociado a la receta no existe".into(),
            ));
        }
    }

    if !repository.unit_exists(input.output_unit_id).await? {
        return Err(AppError::Validation(
            "La unidad de rendimiento de la receta no existe".into(),
        ));
    }

    validate_recipe_status(&input.status)?;
    validate_recipe_items(repository, &input.items).await
}

async fn validate_recipe_update_input(
    repository: &MasterDataRepository<'_>,
    input: &UpdateRecipeInput,
) -> AppResult<()> {
    if input.code.is_empty() || input.name.is_empty() {
        return Err(AppError::Validation(
            "El codigo y el nombre de la receta son obligatorios".into(),
        ));
    }

    if input.version.is_empty() {
        return Err(AppError::Validation(
            "La version de la receta es obligatoria".into(),
        ));
    }

    if input.output_quantity <= 0.0 {
        return Err(AppError::Validation(
            "El rendimiento de la receta debe ser mayor que cero".into(),
        ));
    }

    if let Some(product_id) = input.product_id {
        if !repository.product_exists(product_id).await? {
            return Err(AppError::Validation(
                "El producto asociado a la receta no existe".into(),
            ));
        }
    }

    if !repository.unit_exists(input.output_unit_id).await? {
        return Err(AppError::Validation(
            "La unidad de rendimiento de la receta no existe".into(),
        ));
    }

    validate_recipe_status(&input.status)?;
    validate_recipe_items(repository, &input.items).await
}

fn resolve_conversion_factor(
    from_unit_id: i64,
    to_unit_id: i64,
    conversions: Vec<(i64, i64, f64)>,
) -> AppResult<f64> {
    let mut adjacency_map: HashMap<i64, Vec<(i64, f64)>> = HashMap::new();

    for (from, to, factor) in conversions {
        adjacency_map.entry(from).or_default().push((to, factor));
        adjacency_map.entry(to).or_default().push((from, 1.0 / factor));
    }

    let mut queue = VecDeque::from([(from_unit_id, 1.0_f64)]);
    let mut visited = HashSet::from([from_unit_id]);

    while let Some((current_unit_id, accumulated_factor)) = queue.pop_front() {
        if current_unit_id == to_unit_id {
            return Ok(accumulated_factor);
        }

        if let Some(neighbors) = adjacency_map.get(&current_unit_id) {
            for (next_unit_id, factor) in neighbors {
                if visited.insert(*next_unit_id) {
                    queue.push_back((*next_unit_id, accumulated_factor * factor));
                }
            }
        }
    }

    Err(AppError::Validation(
        "No existe una conversion registrada entre esas unidades".into(),
    ))
}

fn map_material_write_error(error: AppError) -> AppError {
    match error {
        AppError::Database(database_error) => {
            let message = database_error.to_string().to_lowercase();

            if message.contains("unique constraint failed: materials.code") {
                return AppError::Validation(
                    "Ya existe una materia prima con ese codigo".into(),
                );
            }

            if message.contains("foreign key constraint failed") {
                return AppError::Validation(
                    "La materia prima usa unidades invalidas o relacionadas que no existen".into(),
                );
            }

            AppError::Database(database_error)
        }
        _ => error,
    }
}

fn map_unit_write_error(error: AppError) -> AppError {
    match error {
        AppError::Database(database_error) => {
            let message = database_error.to_string().to_lowercase();

            if message.contains("unique constraint failed: units.code") {
                return AppError::Validation("Ya existe una unidad con ese codigo".into());
            }

            if message.contains("foreign key constraint failed") {
                return AppError::Validation(
                    "No se puede eliminar la unidad porque esta siendo usada en materiales, costos, productos o conversiones".into(),
                );
            }

            AppError::Database(database_error)
        }
        _ => error,
    }
}

fn map_product_write_error(error: AppError) -> AppError {
    match error {
        AppError::Database(database_error) => {
            let message = database_error.to_string().to_lowercase();

            if message.contains("unique constraint failed: finished_products.code") {
                return AppError::Validation("Ya existe un producto con ese codigo".into());
            }

            if message.contains("foreign key constraint failed") {
                return AppError::Validation(
                    "El producto usa una unidad o receta invalida".into(),
                );
            }

            AppError::Database(database_error)
        }
        _ => error,
    }
}

fn map_recipe_write_error(error: AppError) -> AppError {
    match error {
        AppError::Database(database_error) => {
            let message = database_error.to_string().to_lowercase();

            if message.contains("unique constraint failed: recipes.code") {
                return AppError::Validation("Ya existe una receta con ese codigo".into());
            }

            if message.contains("foreign key constraint failed") {
                return AppError::Validation(
                    "La receta referencia productos o materias primas que no existen".into(),
                );
            }

            AppError::Database(database_error)
        }
        _ => error,
    }
}

fn map_conversion_write_error(error: AppError) -> AppError {
    match error {
        AppError::Database(database_error) => {
            let message = database_error.to_string().to_lowercase();

            if message.contains("unique constraint failed: unit_conversions.from_unit_id, unit_conversions.to_unit_id") {
                return AppError::Validation(
                    "Ya existe una conversion para ese par de unidades".into(),
                );
            }

            if message.contains("foreign key constraint failed") {
                return AppError::Validation(
                    "La conversion usa unidades que no existen".into(),
                );
            }

            AppError::Database(database_error)
        }
        _ => error,
    }
}

fn validate_product_status(status: &str) -> AppResult<()> {
    match status {
        "active" | "development" | "inactive" => Ok(()),
        _ => Err(AppError::Validation(
            "El estado del producto debe ser active, development o inactive".into(),
        )),
    }
}

fn validate_recipe_status(status: &str) -> AppResult<()> {
    match status {
        "active" | "development" | "inactive" => Ok(()),
        _ => Err(AppError::Validation(
            "El estado de la receta debe ser active, development o inactive".into(),
        )),
    }
}

async fn validate_recipe_items(
    repository: &MasterDataRepository<'_>,
    items: &[RecipeItemPayload],
) -> AppResult<()> {
    if items.is_empty() {
        return Err(AppError::Validation(
            "La receta debe incluir al menos una materia prima".into(),
        ));
    }

    let mut seen_materials = HashSet::new();

    for item in items {
        if item.quantity <= 0.0 {
            return Err(AppError::Validation(
                "Cada item de la receta debe tener una cantidad mayor que cero".into(),
            ));
        }

        if !seen_materials.insert(item.material_id) {
            return Err(AppError::Validation(
                "No puedes repetir la misma materia prima dentro de la receta".into(),
            ));
        }

        if !repository.material_exists(item.material_id).await? {
            return Err(AppError::Validation(
                "La receta contiene una materia prima que no existe".into(),
            ));
        }

        if !repository.unit_exists(item.unit_id).await? {
            return Err(AppError::Validation(
                "La receta contiene una unidad que no existe".into(),
            ));
        }
    }

    Ok(())
}

async fn validate_conversion_input(
    repository: &MasterDataRepository<'_>,
    from_unit_id: i64,
    to_unit_id: i64,
    factor: f64,
) -> AppResult<()> {
    if from_unit_id == to_unit_id {
        return Err(AppError::Validation(
            "La conversion debe usar dos unidades diferentes".into(),
        ));
    }

    if factor <= 0.0 {
        return Err(AppError::Validation(
            "El factor de conversion debe ser mayor que cero".into(),
        ));
    }

    if !repository.unit_exists(from_unit_id).await? || !repository.unit_exists(to_unit_id).await? {
        return Err(AppError::Validation(
            "Las unidades de la conversion no existen".into(),
        ));
    }

    Ok(())
}

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|text| {
        let normalized = text.trim().to_string();
        if normalized.is_empty() {
            None
        } else {
            Some(normalized)
        }
    })
}
