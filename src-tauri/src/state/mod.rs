use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use tauri::{AppHandle, Manager, Runtime};

use crate::db;
use crate::errors::{AppError, AppResult};

pub struct AppState {
    pub app_name: String,
    pub version: String,
    pub environment: String,
    pub pool: SqlitePool,
}

impl AppState {
    pub async fn new<R: Runtime>(app: AppHandle<R>) -> AppResult<Self> {
        let app_data_dir = app
            .path()
            .app_data_dir()
            .map_err(|error| AppError::Configuration(error.to_string()))?;
        std::fs::create_dir_all(&app_data_dir)?;

        let database_path = app_data_dir.join("erp_produccion.sqlite");
        let connection_options = SqliteConnectOptions::new()
            .filename(&database_path)
            .create_if_missing(true)
            .foreign_keys(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(connection_options)
            .await?;

        db::initialize(&pool).await?;

        Ok(Self {
            app_name: String::from("ERP Produccion"),
            version: env!("CARGO_PKG_VERSION").to_string(),
            environment: String::from(if cfg!(debug_assertions) {
                "development"
            } else {
                "production"
            }),
            pool,
        })
    }
}
