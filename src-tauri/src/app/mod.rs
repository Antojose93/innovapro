use tauri::Manager;

use crate::commands;
use crate::state::AppState;

pub fn build() -> tauri::Builder<tauri::Wry> {
    tauri::Builder::default()
        .setup(|app| {
            let app_state = tauri::async_runtime::block_on(AppState::new(app.handle().clone()))?;
            app.manage(app_state);

            if let Some(window) = app.get_webview_window("main") {
                window.set_title("ERP Produccion")?;
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::system::health_check,
            commands::production::list_tabs,
            commands::master_data::get_master_data_snapshot,
            commands::master_data::create_unit,
            commands::master_data::update_unit,
            commands::master_data::delete_unit,
            commands::master_data::create_material,
            commands::master_data::update_material,
            commands::master_data::delete_material,
            commands::master_data::record_material_cost,
            commands::master_data::list_material_cost_history,
            commands::master_data::create_finished_product,
            commands::master_data::create_recipe,
            commands::master_data::update_recipe,
            commands::master_data::delete_recipe,
            commands::master_data::update_finished_product,
            commands::master_data::delete_finished_product,
            commands::master_data::create_unit_conversion,
            commands::master_data::update_unit_conversion,
            commands::master_data::delete_unit_conversion,
            commands::master_data::convert_units
        ])
}
