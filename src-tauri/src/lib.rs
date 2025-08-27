use tauri::Listener;
use tauri_commands::*;

pub mod error;
pub mod injection;
pub mod ipc;
pub mod tauri_commands;
pub mod utils;
use tauri::Manager;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            send_feature_command,
            inject_dll_by_name,
            find_process_by_name,
            is_process_running,
            get_process_info,
            is_module_loaded,
            unload_remote_module
            ])
        .setup(|app| {
             let main_window = app.get_webview_window("main").unwrap();

             let win = main_window.clone();
             app.listen("frontend-ready", move |_| {
                 let _ = win.show();
             });
         
             Ok(())
        }).run(tauri::generate_context!())
        .expect("error while running tauri application");
}
