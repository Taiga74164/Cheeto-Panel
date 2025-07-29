use crate::error::AppError;
use std::ffi::CString;
use winapi::um::errhandlingapi::GetLastError;

pub fn create_cstring(s: &str) -> Result<CString, AppError> {
    CString::new(s).map_err(|_| AppError::Process {
        message: "Invalid string for CString conversion".to_string(),
    })
}

pub fn get_last_windows_error() -> AppError {
    let code = unsafe { GetLastError() };
    AppError::WindowsApi { code }
}
