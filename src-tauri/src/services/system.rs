use crate::state::AppState;

pub struct SystemService<'a> {
    state: &'a AppState,
}

pub struct SystemHealth {
    pub app_name: String,
    pub version: String,
    pub environment: String,
    pub status: String,
}

impl<'a> SystemService<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }

    pub fn health_check(&self) -> SystemHealth {
        SystemHealth {
            app_name: self.state.app_name.clone(),
            version: self.state.version.clone(),
            environment: self.state.environment.clone(),
            status: String::from("ok"),
        }
    }
}
