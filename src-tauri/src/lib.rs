use tauri_commands::*;

pub mod error;
pub mod injection;
pub mod ipc;
pub mod tauri_commands;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            send_feature_command,
            inject_dll_by_name,
            find_process_by_name
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
