use crate::error::AppError;
use crate::utils::windows_utils::get_last_windows_error;
use winapi::um::{
    handleapi::{CloseHandle, INVALID_HANDLE_VALUE},
    tlhelp32::{
        CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
    },
    winnt::{HANDLE},
};

pub struct ProcessUtils;

impl ProcessUtils {
    pub fn get_pid(process_name: &str) -> Result<u32, AppError> {
        let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
        if snapshot == INVALID_HANDLE_VALUE {
            return Err(AppError::Process {
                message: "Failed to create snapshot".to_string(),
            });
        }

        let mut pe32: PROCESSENTRY32 = unsafe { std::mem::zeroed() };
        pe32.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

        if unsafe { Process32First(snapshot, &mut pe32) } == 0 {
            unsafe { CloseHandle(snapshot) };
            return Err(AppError::Process {
                message: "Failed to retrieve first process".to_string(),
            });
        }

        loop {
            if process_name
                == unsafe { std::ffi::CStr::from_ptr(pe32.szExeFile.as_ptr()) }
                    .to_str()
                    .unwrap_or("")
            {
                unsafe { CloseHandle(snapshot) };
                return Ok(pe32.th32ProcessID);
            }

            if unsafe { Process32Next(snapshot, &mut pe32) } == 0 {
                break;
            }
        }

        unsafe { CloseHandle(snapshot) };
        Err(AppError::Process {
            message: format!("Process '{}' not found", process_name),
        })
    }

    // pub fn get_module_handle_in_process(process_handle: HANDLE, module_name: &str) -> Result<HMODULE, AppError> {}

    pub fn open_process(process_id: u32, access_rights: u32) -> Result<HANDLE, AppError> {
        let handle =
            unsafe { winapi::um::processthreadsapi::OpenProcess(access_rights, 0, process_id) };
        if handle.is_null() {
            return Err(AppError::Process {
                message: get_last_windows_error().to_string(),
            });
        }

        Ok(handle)
    }
}
