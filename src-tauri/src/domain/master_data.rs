#[derive(Clone, Debug, serde::Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Unit {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub symbol: String,
}

#[derive(Clone, Debug, serde::Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Material {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub stock_unit_id: i64,
    pub purchase_unit_id: i64,
    pub current_cost: Option<f64>,
    pub current_cost_unit_id: Option<i64>,
    pub last_cost_at: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct MaterialCostHistoryEntry {
    pub id: i64,
    pub material_id: i64,
    pub unit_id: i64,
    pub unit_cost: f64,
    pub effective_at: String,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FinishedProduct {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub unit_id: i64,
    pub recipe_id: Option<i64>,
    pub category: String,
    pub status: String,
    pub internal_code: Option<String>,
    pub commercial_code: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct RecipeItem {
    pub recipe_id: i64,
    pub material_id: i64,
    pub quantity: f64,
    pub unit_id: i64,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeItemPayload {
    pub material_id: i64,
    pub quantity: f64,
    pub unit_id: i64,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub product_id: Option<i64>,
    pub status: String,
    pub version: String,
    pub output_quantity: f64,
    pub output_unit_id: i64,
    pub notes: Option<String>,
    pub items: Vec<RecipeItemPayload>,
}

#[derive(Clone, Debug, serde::Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UnitConversion {
    pub id: i64,
    pub from_unit_id: i64,
    pub to_unit_id: i64,
    pub factor: f64,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MasterDataSnapshot {
    pub units: Vec<Unit>,
    pub materials: Vec<Material>,
    pub products: Vec<FinishedProduct>,
    pub recipes: Vec<Recipe>,
    pub conversions: Vec<UnitConversion>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUnitInput {
    pub code: String,
    pub name: String,
    pub symbol: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUnitInput {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub symbol: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMaterialInput {
    pub code: String,
    pub name: String,
    pub stock_unit_id: i64,
    pub purchase_unit_id: i64,
    pub initial_cost: Option<f64>,
    pub initial_cost_unit_id: Option<i64>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMaterialInput {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub stock_unit_id: i64,
    pub purchase_unit_id: i64,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordMaterialCostInput {
    pub material_id: i64,
    pub unit_id: i64,
    pub unit_cost: f64,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFinishedProductInput {
    pub code: String,
    pub name: String,
    pub unit_id: i64,
    pub recipe_id: Option<i64>,
    pub category: String,
    pub status: String,
    pub internal_code: Option<String>,
    pub commercial_code: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFinishedProductInput {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub unit_id: i64,
    pub recipe_id: Option<i64>,
    pub category: String,
    pub status: String,
    pub internal_code: Option<String>,
    pub commercial_code: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecipeInput {
    pub code: String,
    pub name: String,
    pub product_id: Option<i64>,
    pub status: String,
    pub version: String,
    pub output_quantity: f64,
    pub output_unit_id: i64,
    pub notes: Option<String>,
    pub items: Vec<RecipeItemPayload>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRecipeInput {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub product_id: Option<i64>,
    pub status: String,
    pub version: String,
    pub output_quantity: f64,
    pub output_unit_id: i64,
    pub notes: Option<String>,
    pub items: Vec<RecipeItemPayload>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUnitConversionInput {
    pub from_unit_id: i64,
    pub to_unit_id: i64,
    pub factor: f64,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUnitConversionInput {
    pub id: i64,
    pub from_unit_id: i64,
    pub to_unit_id: i64,
    pub factor: f64,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertUnitsInput {
    pub from_unit_id: i64,
    pub to_unit_id: i64,
    pub quantity: f64,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversionResult {
    pub factor: f64,
    pub converted_quantity: f64,
}
