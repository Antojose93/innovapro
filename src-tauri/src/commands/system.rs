use crate::errors::AppResult;
use crate::services::system::SystemService;
use crate::state::AppState;

#[tauri::command]
pub fn health_check(state: tauri::State<'_, AppState>) -> AppResult<SystemHealthResponse> {
    Ok(SystemService::new(state.inner())
        .health_check()
        .into())
}

#[derive(serde::Serialize)]
pub struct SystemHealthResponse {
    pub app_name: String,
    pub version: String,
    pub environment: String,
    pub status: String,
}

impl From<crate::services::system::SystemHealth> for SystemHealthResponse {
    fn from(value: crate::services::system::SystemHealth) -> Self {
        Self {
            app_name: value.app_name,
            version: value.version,
            environment: value.environment,
            status: value.status,
        }
    }
}
