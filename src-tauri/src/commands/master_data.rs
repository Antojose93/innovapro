use crate::domain::master_data::{
    ConversionResult, ConvertUnitsInput, CreateFinishedProductInput, CreateMaterialInput,
    CreateRecipeInput, CreateUnitConversionInput, CreateUnitInput, MasterDataSnapshot,
    MaterialCostHistoryEntry, RecordMaterialCostInput, UpdateFinishedProductInput,
    UpdateMaterialInput, UpdateRecipeInput, UpdateUnitConversionInput, UpdateUnitInput,
};
use crate::errors::AppResult;
use crate::services::master_data::MasterDataService;
use crate::state::AppState;

#[tauri::command]
pub async fn get_master_data_snapshot(
    state: tauri::State<'_, AppState>,
) -> AppResult<MasterDataSnapshot> {
    MasterDataService::new(state.inner()).get_snapshot().await
}

#[tauri::command]
pub async fn create_unit(
    state: tauri::State<'_, AppState>,
    input: CreateUnitInput,
) -> AppResult<MasterDataSnapshot> {
    MasterDataService::new(state.inner()).create_unit(input).await
}

#[tauri::command]
pub async fn update_unit(
    state: tauri::State<'_, AppState>,
    input: UpdateUnitInput,
) -> AppResult<MasterDataSnapshot> {
    MasterDataService::new(state.inner()).update_unit(input).await
}

#[tauri::command]
pub async fn delete_unit(
    state: tauri::State<'_, AppState>,
    unit_id: i64,
) -> AppResult<MasterDataSnapshot> {
    MasterDataService::new(state.inner()).delete_unit(unit_id).await
}

#[tauri::command]
pub async fn create_material(
    state: tauri::State<'_, AppState>,
    input: CreateMaterialInput,
) -> AppResult<MasterDataSnapshot> {
    MasterDataService::new(state.inner()).create_material(input).await
}

#[tauri::command]
pub async fn update_material(
    state: tauri::State<'_, AppState>,
    input: UpdateMaterialInput,
) -> AppResult<MasterDataSnapshot> {
    MasterDataService::new(state.inner()).update_material(input).await
}

#[tauri::command]
pub async fn delete_material(
    state: tauri::State<'_, AppState>,
    material_id: i64,
) -> AppResult<MasterDataSnapshot> {
    MasterDataService::new(state.inner()).delete_material(material_id).await
}

#[tauri::command]
pub async fn record_material_cost(
    state: tauri::State<'_, AppState>,
    input: RecordMaterialCostInput,
) -> AppResult<Vec<MaterialCostHistoryEntry>> {
    MasterDataService::new(state.inner())
        .record_material_cost(input)
        .await
}

#[tauri::command]
pub async fn list_material_cost_history(
    state: tauri::State<'_, AppState>,
    material_id: i64,
) -> AppResult<Vec<MaterialCostHistoryEntry>> {
    MasterDataService::new(state.inner())
        .list_material_cost_history(material_id)
        .await
}

#[tauri::command]
pub async fn create_finished_product(
    state: tauri::State<'_, AppState>,
    input: CreateFinishedProductInput,
) -> AppResult<MasterDataSnapshot> {
    MasterDataService::new(state.inner())
        .create_finished_product(input)
        .await
}

#[tauri::command]
pub async fn create_recipe(
    state: tauri::State<'_, AppState>,
    input: CreateRecipeInput,
) -> AppResult<MasterDataSnapshot> {
    MasterDataService::new(state.inner()).create_recipe(input).await
}

#[tauri::command]
pub async fn update_recipe(
    state: tauri::State<'_, AppState>,
    input: UpdateRecipeInput,
) -> AppResult<MasterDataSnapshot> {
    MasterDataService::new(state.inner()).update_recipe(input).await
}

#[tauri::command]
pub async fn delete_recipe(
    state: tauri::State<'_, AppState>,
    recipe_id: i64,
) -> AppResult<MasterDataSnapshot> {
    MasterDataService::new(state.inner()).delete_recipe(recipe_id).await
}

#[tauri::command]
pub async fn update_finished_product(
    state: tauri::State<'_, AppState>,
    input: UpdateFinishedProductInput,
) -> AppResult<MasterDataSnapshot> {
    MasterDataService::new(state.inner())
        .update_finished_product(input)
        .await
}

#[tauri::command]
pub async fn delete_finished_product(
    state: tauri::State<'_, AppState>,
    product_id: i64,
) -> AppResult<MasterDataSnapshot> {
    MasterDataService::new(state.inner())
        .delete_finished_product(product_id)
        .await
}

#[tauri::command]
pub async fn create_unit_conversion(
    state: tauri::State<'_, AppState>,
    input: CreateUnitConversionInput,
) -> AppResult<MasterDataSnapshot> {
    MasterDataService::new(state.inner())
        .create_unit_conversion(input)
        .await
}

#[tauri::command]
pub async fn update_unit_conversion(
    state: tauri::State<'_, AppState>,
    input: UpdateUnitConversionInput,
) -> AppResult<MasterDataSnapshot> {
    MasterDataService::new(state.inner())
        .update_unit_conversion(input)
        .await
}

#[tauri::command]
pub async fn delete_unit_conversion(
    state: tauri::State<'_, AppState>,
    conversion_id: i64,
) -> AppResult<MasterDataSnapshot> {
    MasterDataService::new(state.inner())
        .delete_unit_conversion(conversion_id)
        .await
}

#[tauri::command]
pub async fn convert_units(
    state: tauri::State<'_, AppState>,
    input: ConvertUnitsInput,
) -> AppResult<ConversionResult> {
    MasterDataService::new(state.inner()).convert_units(input).await
}
