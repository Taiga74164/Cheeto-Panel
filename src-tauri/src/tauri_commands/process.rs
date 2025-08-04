use winapi::um::winnt::{
    PROCESS_QUERY_INFORMATION, PROCESS_VM_READ,
};

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

#[tauri::command]
pub async fn is_module_loaded(process_name: String, module_name: String) -> Result<bool, String> {
    let pid = ProcessUtils::get_pid(&process_name)?;
    let h_process = ProcessUtils::open_process(pid, PROCESS_QUERY_INFORMATION | PROCESS_VM_READ)?;

    let is_loaded = ProcessUtils::get_module_handle_in_process(pid, &module_name).is_ok();

    unsafe { winapi::um::handleapi::CloseHandle(h_process) };

    Ok(is_loaded)
}

#[tauri::command]
pub async fn unload_remote_module(process_name: String, module_name: String) -> Result<(), String> {
    let pid = ProcessUtils::get_pid(&process_name)?;

    let result = ProcessUtils::unload_dll(pid, &module_name);
    if result.is_err() {
        return Err(result.err().unwrap().to_string());
    }

    Ok(())
}
