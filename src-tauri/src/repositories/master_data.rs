use sqlx::{Row, SqlitePool};

use crate::domain::master_data::{
    CreateFinishedProductInput, CreateMaterialInput, CreateRecipeInput,
    CreateUnitConversionInput, CreateUnitInput, FinishedProduct, Material,
    MaterialCostHistoryEntry, Recipe, RecipeItem, RecipeItemPayload, RecordMaterialCostInput,
    Unit, UnitConversion, UpdateFinishedProductInput, UpdateMaterialInput, UpdateRecipeInput,
    UpdateUnitConversionInput, UpdateUnitInput,
};
use crate::errors::AppResult;

pub struct MasterDataRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> MasterDataRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn list_units(&self) -> AppResult<Vec<Unit>> {
        Ok(sqlx::query_as::<_, Unit>(
            r#"
            SELECT id, code, name, symbol
            FROM units
            ORDER BY name ASC
            "#,
        )
        .fetch_all(self.pool)
        .await?)
    }

    pub async fn list_materials(&self) -> AppResult<Vec<Material>> {
        Ok(sqlx::query_as::<_, Material>(
            r#"
            SELECT
              materials.id,
              materials.code,
              materials.name,
              materials.stock_unit_id,
              materials.purchase_unit_id,
              latest.unit_cost AS current_cost,
              latest.unit_id AS current_cost_unit_id,
              latest.effective_at AS last_cost_at
            FROM materials
            LEFT JOIN (
              SELECT mch.material_id, mch.unit_id, mch.unit_cost, mch.effective_at
              FROM material_cost_history mch
              INNER JOIN (
                SELECT material_id, MAX(id) AS latest_id
                FROM material_cost_history
                GROUP BY material_id
              ) grouped ON grouped.latest_id = mch.id
            ) latest ON latest.material_id = materials.id
            ORDER BY materials.name ASC
            "#,
        )
        .fetch_all(self.pool)
        .await?)
    }

    pub async fn list_material_cost_history(
        &self,
        material_id: i64,
    ) -> AppResult<Vec<MaterialCostHistoryEntry>> {
        Ok(sqlx::query_as::<_, MaterialCostHistoryEntry>(
            r#"
            SELECT id, material_id, unit_id, unit_cost, effective_at
            FROM material_cost_history
            WHERE material_id = ?
            ORDER BY id DESC
            "#,
        )
        .bind(material_id)
        .fetch_all(self.pool)
        .await?)
    }

    pub async fn list_products(&self) -> AppResult<Vec<FinishedProduct>> {
        #[derive(sqlx::FromRow)]
        struct FinishedProductRow {
            id: i64,
            code: String,
            name: String,
            unit_id: i64,
            recipe_id: Option<i64>,
            category: String,
            status: String,
            internal_code: Option<String>,
            commercial_code: Option<String>,
        }

        let rows = sqlx::query_as::<_, FinishedProductRow>(
            r#"
            SELECT id, code, name, unit_id, recipe_id, category, status, internal_code, commercial_code
            FROM finished_products
            ORDER BY name ASC
            "#,
        )
        .fetch_all(self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| FinishedProduct {
                id: row.id,
                code: row.code,
                name: row.name,
                unit_id: row.unit_id,
                recipe_id: row.recipe_id,
                category: row.category,
                status: row.status,
                internal_code: row.internal_code,
                commercial_code: row.commercial_code,
            })
            .collect())
    }

    pub async fn list_recipes(&self) -> AppResult<Vec<Recipe>> {
        #[derive(sqlx::FromRow)]
        struct RecipeRow {
            id: i64,
            code: String,
            name: String,
            product_id: Option<i64>,
            status: String,
            version: String,
            output_quantity: f64,
            output_unit_id: i64,
            notes: Option<String>,
        }

        let rows = sqlx::query_as::<_, RecipeRow>(
            r#"
            SELECT id, code, name, product_id, status, version, output_quantity, output_unit_id, notes
            FROM recipes
            ORDER BY name ASC
            "#,
        )
        .fetch_all(self.pool)
        .await?;

        let recipe_items = self.list_recipe_items().await?;

        Ok(rows
            .into_iter()
            .map(|row| Recipe {
                id: row.id,
                code: row.code,
                name: row.name,
                product_id: row.product_id,
                status: row.status,
                version: row.version,
                output_quantity: row.output_quantity,
                output_unit_id: row.output_unit_id,
                notes: row.notes,
                items: recipe_items
                    .iter()
                    .filter(|item| item.recipe_id == row.id)
                    .map(|item| RecipeItemPayload {
                        material_id: item.material_id,
                        quantity: item.quantity,
                        unit_id: item.unit_id,
                    })
                    .collect(),
            })
            .collect())
    }

    pub async fn list_recipe_items(&self) -> AppResult<Vec<RecipeItem>> {
        Ok(sqlx::query_as::<_, RecipeItem>(
            r#"
            SELECT recipe_id, material_id, quantity, unit_id
            FROM recipe_items
            ORDER BY recipe_id ASC, id ASC
            "#,
        )
        .fetch_all(self.pool)
        .await?)
    }

    pub async fn list_conversions(&self) -> AppResult<Vec<UnitConversion>> {
        Ok(sqlx::query_as::<_, UnitConversion>(
            r#"
            SELECT id, from_unit_id, to_unit_id, factor
            FROM unit_conversions
            ORDER BY id DESC
            "#,
        )
        .fetch_all(self.pool)
        .await?)
    }

    pub async fn create_unit(&self, input: &CreateUnitInput) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO units (code, name, symbol)
            VALUES (?, ?, ?)
            "#,
        )
        .bind(&input.code)
        .bind(&input.name)
        .bind(&input.symbol)
        .execute(self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_unit(&self, input: &UpdateUnitInput) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE units
            SET code = ?, name = ?, symbol = ?
            WHERE id = ?
            "#,
        )
        .bind(&input.code)
        .bind(&input.name)
        .bind(&input.symbol)
        .bind(input.id)
        .execute(self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_unit(&self, unit_id: i64) -> AppResult<()> {
        sqlx::query("DELETE FROM units WHERE id = ?")
            .bind(unit_id)
            .execute(self.pool)
            .await?;

        Ok(())
    }

    pub async fn create_material(
        &self,
        input: &CreateMaterialInput,
        initial_cost_unit_id: Option<i64>,
    ) -> AppResult<()> {
        let mut transaction = self.pool.begin().await?;

        let result = sqlx::query(
            r#"
            INSERT INTO materials (code, name, stock_unit_id, purchase_unit_id)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(&input.code)
        .bind(&input.name)
        .bind(input.stock_unit_id)
        .bind(input.purchase_unit_id)
        .execute(&mut *transaction)
        .await?;

        if let Some(initial_cost) = input.initial_cost {
            sqlx::query(
                r#"
                INSERT INTO material_cost_history (material_id, unit_id, unit_cost)
                VALUES (?, ?, ?)
                "#,
            )
            .bind(result.last_insert_rowid())
            .bind(initial_cost_unit_id.unwrap_or(input.purchase_unit_id))
            .bind(initial_cost)
            .execute(&mut *transaction)
            .await?;
        }

        transaction.commit().await?;

        Ok(())
    }

    pub async fn record_material_cost(&self, input: &RecordMaterialCostInput) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO material_cost_history (material_id, unit_id, unit_cost)
            VALUES (?, ?, ?)
            "#,
        )
        .bind(input.material_id)
        .bind(input.unit_id)
        .bind(input.unit_cost)
        .execute(self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_material(&self, input: &UpdateMaterialInput) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE materials
            SET code = ?, name = ?, stock_unit_id = ?, purchase_unit_id = ?
            WHERE id = ?
            "#,
        )
        .bind(&input.code)
        .bind(&input.name)
        .bind(input.stock_unit_id)
        .bind(input.purchase_unit_id)
        .bind(input.id)
        .execute(self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_material(&self, material_id: i64) -> AppResult<()> {
        sqlx::query("DELETE FROM materials WHERE id = ?")
            .bind(material_id)
            .execute(self.pool)
            .await?;

        Ok(())
    }

    pub async fn create_finished_product(&self, input: &CreateFinishedProductInput) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO finished_products (code, name, unit_id, recipe_id, category, status, internal_code, commercial_code)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&input.code)
        .bind(&input.name)
        .bind(input.unit_id)
        .bind(input.recipe_id)
        .bind(&input.category)
        .bind(&input.status)
        .bind(&input.internal_code)
        .bind(&input.commercial_code)
        .execute(self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_finished_product(
        &self,
        input: &UpdateFinishedProductInput,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE finished_products
            SET code = ?, name = ?, unit_id = ?, recipe_id = ?, category = ?, status = ?, internal_code = ?, commercial_code = ?
            WHERE id = ?
            "#,
        )
        .bind(&input.code)
        .bind(&input.name)
        .bind(input.unit_id)
        .bind(input.recipe_id)
        .bind(&input.category)
        .bind(&input.status)
        .bind(&input.internal_code)
        .bind(&input.commercial_code)
        .bind(input.id)
        .execute(self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_finished_product(&self, product_id: i64) -> AppResult<()> {
        sqlx::query("DELETE FROM finished_products WHERE id = ?")
            .bind(product_id)
            .execute(self.pool)
            .await?;

        Ok(())
    }

    pub async fn create_unit_conversion(
        &self,
        input: &CreateUnitConversionInput,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO unit_conversions (from_unit_id, to_unit_id, factor)
            VALUES (?, ?, ?)
            ON CONFLICT (from_unit_id, to_unit_id)
            DO UPDATE SET factor = excluded.factor
            "#,
        )
        .bind(input.from_unit_id)
        .bind(input.to_unit_id)
        .bind(input.factor)
        .execute(self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_unit_conversion(
        &self,
        input: &UpdateUnitConversionInput,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE unit_conversions
            SET from_unit_id = ?, to_unit_id = ?, factor = ?
            WHERE id = ?
            "#,
        )
        .bind(input.from_unit_id)
        .bind(input.to_unit_id)
        .bind(input.factor)
        .bind(input.id)
        .execute(self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_unit_conversion(&self, conversion_id: i64) -> AppResult<()> {
        sqlx::query("DELETE FROM unit_conversions WHERE id = ?")
            .bind(conversion_id)
            .execute(self.pool)
            .await?;

        Ok(())
    }

    pub async fn create_recipe(&self, input: &CreateRecipeInput) -> AppResult<()> {
        let mut transaction = self.pool.begin().await?;

        let result = sqlx::query(
            r#"
            INSERT INTO recipes (code, name, product_id, status, version, output_quantity, output_unit_id, notes)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&input.code)
        .bind(&input.name)
        .bind(input.product_id)
        .bind(&input.status)
        .bind(&input.version)
        .bind(input.output_quantity)
        .bind(input.output_unit_id)
        .bind(&input.notes)
        .execute(&mut *transaction)
        .await?;

        for item in &input.items {
            sqlx::query(
                r#"
                INSERT INTO recipe_items (recipe_id, material_id, quantity, unit_id)
                VALUES (?, ?, ?, ?)
                "#,
            )
            .bind(result.last_insert_rowid())
            .bind(item.material_id)
            .bind(item.quantity)
            .bind(item.unit_id)
            .execute(&mut *transaction)
            .await?;
        }

        transaction.commit().await?;
        Ok(())
    }

    pub async fn update_recipe(&self, input: &UpdateRecipeInput) -> AppResult<()> {
        let mut transaction = self.pool.begin().await?;

        sqlx::query(
            r#"
            UPDATE recipes
            SET code = ?, name = ?, product_id = ?, status = ?, version = ?, output_quantity = ?, output_unit_id = ?, notes = ?
            WHERE id = ?
            "#,
        )
        .bind(&input.code)
        .bind(&input.name)
        .bind(input.product_id)
        .bind(&input.status)
        .bind(&input.version)
        .bind(input.output_quantity)
        .bind(input.output_unit_id)
        .bind(&input.notes)
        .bind(input.id)
        .execute(&mut *transaction)
        .await?;

        sqlx::query("DELETE FROM recipe_items WHERE recipe_id = ?")
            .bind(input.id)
            .execute(&mut *transaction)
            .await?;

        for item in &input.items {
            sqlx::query(
                r#"
                INSERT INTO recipe_items (recipe_id, material_id, quantity, unit_id)
                VALUES (?, ?, ?, ?)
                "#,
            )
            .bind(input.id)
            .bind(item.material_id)
            .bind(item.quantity)
            .bind(item.unit_id)
            .execute(&mut *transaction)
            .await?;
        }

        transaction.commit().await?;
        Ok(())
    }

    pub async fn delete_recipe(&self, recipe_id: i64) -> AppResult<()> {
        sqlx::query("DELETE FROM recipes WHERE id = ?")
            .bind(recipe_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn unit_exists(&self, unit_id: i64) -> AppResult<bool> {
        let record = sqlx::query("SELECT EXISTS(SELECT 1 FROM units WHERE id = ?) AS exists_flag")
            .bind(unit_id)
            .fetch_one(self.pool)
            .await?;

        Ok(record.get::<i64, _>("exists_flag") == 1)
    }

    pub async fn material_exists(&self, material_id: i64) -> AppResult<bool> {
        let record =
            sqlx::query("SELECT EXISTS(SELECT 1 FROM materials WHERE id = ?) AS exists_flag")
                .bind(material_id)
                .fetch_one(self.pool)
                .await?;

        Ok(record.get::<i64, _>("exists_flag") == 1)
    }

    pub async fn product_exists(&self, product_id: i64) -> AppResult<bool> {
        let record = sqlx::query(
            "SELECT EXISTS(SELECT 1 FROM finished_products WHERE id = ?) AS exists_flag",
        )
        .bind(product_id)
        .fetch_one(self.pool)
        .await?;

        Ok(record.get::<i64, _>("exists_flag") == 1)
    }

    pub async fn recipe_exists(&self, recipe_id: i64) -> AppResult<bool> {
        let record = sqlx::query("SELECT EXISTS(SELECT 1 FROM recipes WHERE id = ?) AS exists_flag")
            .bind(recipe_id)
            .fetch_one(self.pool)
            .await?;

        Ok(record.get::<i64, _>("exists_flag") == 1)
    }

    pub async fn conversion_exists(&self, conversion_id: i64) -> AppResult<bool> {
        let record = sqlx::query(
            "SELECT EXISTS(SELECT 1 FROM unit_conversions WHERE id = ?) AS exists_flag",
        )
        .bind(conversion_id)
        .fetch_one(self.pool)
        .await?;

        Ok(record.get::<i64, _>("exists_flag") == 1)
    }
}
