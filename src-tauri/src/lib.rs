use std::{ffi::OsStr, os::windows::ffi::OsStrExt, ptr, time::Duration};
use winapi::{
    shared::winerror::ERROR_PIPE_BUSY,
    um::{
        fileapi::{CreateFileW, OPEN_EXISTING},
        handleapi::{CloseHandle, INVALID_HANDLE_VALUE},
        namedpipeapi::SetNamedPipeHandleState,
        winbase::PIPE_READMODE_MESSAGE,
        winnt::{FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ, GENERIC_WRITE},
    },
};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

struct NamedPipe {
    handle: *mut winapi::ctypes::c_void,
}

impl NamedPipe {
    pub const PIPE_NAME: &'static str= r"\\.\pipe\cunny_pipe00";

    fn connect(pipe_name: &str) -> Result<Self, String> {
        let pipe_name_wide: Vec<u16> = OsStr::new(pipe_name)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let mut attempts = 0;
        let max_attempts = 10;

        loop {
            let handle = unsafe {
                CreateFileW(
                    pipe_name_wide.as_ptr(),
                    GENERIC_READ | GENERIC_WRITE,
                    FILE_SHARE_READ | FILE_SHARE_WRITE,
                    ptr::null_mut(),
                    OPEN_EXISTING,
                    0,
                    ptr::null_mut(),
                )
            };

            if handle != INVALID_HANDLE_VALUE {
                let mut mode = PIPE_READMODE_MESSAGE;
                let result = unsafe {
                    SetNamedPipeHandleState(handle, &mut mode, ptr::null_mut(), ptr::null_mut())
                };

                if result == 0 {
                    unsafe { CloseHandle(handle) };
                    return Err("Failed to set pipe mode".to_string());
                }

                return Ok(NamedPipe { handle });
            }

            let error = unsafe { winapi::um::errhandlingapi::GetLastError() };
            if error == ERROR_PIPE_BUSY {
                // Pipe is busy, wait and retry
                std::thread::sleep(Duration::from_millis(100));
                attempts += 1;
                if attempts >= max_attempts {
                    return Err("Pipe is busy, max attempts reached".to_string());
                }
                continue;
            } else {
                return Err(format!("Failed to connect to pipe: error code {}", error));
            }
        }
    }

    fn write(&mut self, data: &[u8]) -> Result<(), String> {
        let mut bytes_written = 0u32;
        let result = unsafe {
            winapi::um::fileapi::WriteFile(
                self.handle,
                data.as_ptr() as *const winapi::ctypes::c_void,
                data.len() as u32,
                &mut bytes_written,
                ptr::null_mut(),
            )
        };

        if result == 0 {
            let error = unsafe { winapi::um::errhandlingapi::GetLastError() };
            return Err(format!("Failed to write to pipe: error code {}", error));
        }

        if bytes_written != data.len() as u32 {
            return Err("Not all data was written to pipe".to_string());
        }

        // Flush the pipe
        unsafe { winapi::um::fileapi::FlushFileBuffers(self.handle) };

        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, String> {
        let mut bytes_read = 0u32;
        let result = unsafe {
            winapi::um::fileapi::ReadFile(
                self.handle,
                buffer.as_mut_ptr() as *mut winapi::ctypes::c_void,
                buffer.len() as u32,
                &mut bytes_read,
                ptr::null_mut(),
            )
        };

        if result == 0 {
            let error = unsafe { winapi::um::errhandlingapi::GetLastError() };
            return Err(format!("Failed to read from pipe: error code {}", error));
        }

        Ok(bytes_read as usize)
    }
}

impl Drop for NamedPipe {
    fn drop(&mut self) {
        if self.handle != INVALID_HANDLE_VALUE {
            unsafe { CloseHandle(self.handle) };
        }
    }
}

#[tauri::command]
async fn send_feature_command(feature: String, enable: bool) -> Result<String, String> {

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, send_feature_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
