use crate::ipc::{NamedPipe};
use crate::injection::{DllInjector, ProcessUtils, SharedMemoryFlag};
use winapi::um::winnt::PROCESS_ALL_ACCESS;

#[tauri::command]
pub async fn send_feature_command(feature: String, enable: bool) -> Result<String, String> {
    let mut pipe = NamedPipe::connect(NamedPipe::PIPE_NAME)?;

    let message = format!(
        "feature:{}:{}",
        feature,
        if enable { "enable" } else { "disable" }
    );

    pipe.write(message.as_bytes())?;

    let mut buffer = [0u8; 1024];
    let bytes_read = pipe.read(&mut buffer)?;

    let response = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
    Ok(response)
}

#[tauri::command]
pub async fn inject_dll_by_name(process_name: String, dll_path: String, use_pipe_mode: bool) -> Result<String, String> {
    let process_id = ProcessUtils::get_pid(&process_name)?;

    let h_process = ProcessUtils::open_process(process_id, PROCESS_ALL_ACCESS)?;

    let _keep_flag_alive = SharedMemoryFlag::new(use_pipe_mode, process_id)?;

    DllInjector::inject_dll(h_process, &dll_path)?;

    unsafe { winapi::um::handleapi::CloseHandle(h_process) };

    Ok(format!(
        "DLL '{}' injected into process '{}' (PID: {})",
        dll_path, process_name, process_id
    ))
}

#[tauri::command]
pub async fn find_process_by_name(process_name: String) -> Result<u32, String> {
    let pid = ProcessUtils::get_pid(&process_name)?;
    Ok(pid)
}
