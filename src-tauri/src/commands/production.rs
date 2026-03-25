use crate::errors::AppResult;
use crate::services::production::ProductionService;
use crate::state::AppState;

#[tauri::command]
pub fn list_tabs(state: tauri::State<'_, AppState>) -> AppResult<Vec<ProductionTabResponse>> {
    Ok(ProductionService::new(state.inner())
        .list_tabs()
        .into_iter()
        .map(Into::into)
        .collect())
}

#[derive(serde::Serialize)]
pub struct ProductionTabResponse {
    pub key: String,
    pub label: String,
    pub icon: String,
    pub route: String,
}

impl From<crate::services::production::ProductionTab> for ProductionTabResponse {
    fn from(value: crate::services::production::ProductionTab) -> Self {
        Self {
            key: value.key,
            label: value.label,
            icon: value.icon,
            route: value.route,
        }
    }
}
