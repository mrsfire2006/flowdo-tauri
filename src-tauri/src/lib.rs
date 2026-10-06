// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;

mod auth;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
    
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            let auth_state = tauri::async_runtime::block_on(auth::AuthState::initialize(&data_dir))
                .map_err(std::io::Error::other)?;
            app.manage(auth_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            auth::auth_sign_up,
            auth::auth_sign_in,
            auth::auth_get_session,
            auth::auth_sign_out,
            auth::task_list,
            auth::task_create,
            auth::task_update,
            auth::task_delete
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
