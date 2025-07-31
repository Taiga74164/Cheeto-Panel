use crate::injection::{ProcessInfo, ProcessUtils};

#[tauri::command]
pub async fn find_process_by_name(process_name: String) -> Result<u32, String> {
    let pid = ProcessUtils::get_pid(&process_name)?;
    Ok(pid)
}

#[tauri::command]
pub async fn is_process_running(process_name: String) -> Result<bool, String> {
    let pid = ProcessUtils::get_pid(&process_name);
    match pid {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[tauri::command]
pub async fn get_process_info(process_name: String) -> Result<ProcessInfo, String> {
    match ProcessUtils::get_pid(&process_name) {
        Ok(pid) => Ok(ProcessInfo {
            name: process_name,
            pid,
            is_running: true,
        }),
        Err(e) => Err(e.to_string()),
    }
}