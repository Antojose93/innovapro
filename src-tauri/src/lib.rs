mod app;
mod commands;
mod db;
mod domain;
mod errors;
mod repositories;
mod services;
mod state;

pub fn run() {
    app::build()
        .run(tauri::generate_context!())
        .expect("failed to run tauri application");
}
