use crate::error::AppError;
use crate::utils::windows_utils::get_last_windows_error;
use std::{ffi::OsStr, os::windows::ffi::OsStrExt, ptr, time::Duration};
use winapi::{
    shared::winerror::ERROR_PIPE_BUSY,
    um::{
        fileapi::{CreateFileW, FlushFileBuffers, ReadFile, WriteFile, OPEN_EXISTING},
        handleapi::{CloseHandle, INVALID_HANDLE_VALUE},
        namedpipeapi::SetNamedPipeHandleState,
        winbase::PIPE_READMODE_MESSAGE,
        winnt::{FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ, GENERIC_WRITE, HANDLE},
    },
};

pub struct NamedPipe {
    handle: HANDLE,
}

impl NamedPipe {
    pub const PIPE_NAME: &'static str = r"\\.\pipe\cunny_pipe00";

    pub fn connect(pipe_name: &str) -> Result<Self, AppError> {
        let pipe_name_wide: Vec<u16> = OsStr::new(pipe_name)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let mut attempts = 0;
        const MAX_ATTEMPTS: u32 = 10;

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
                    return Err(AppError::Ipc {
                        message: "Failed to set pipe mode".to_string(),
                    });
                }

                return Ok(NamedPipe { handle });
            }

            let error = unsafe { winapi::um::errhandlingapi::GetLastError() };
            if error == ERROR_PIPE_BUSY {
                std::thread::sleep(Duration::from_millis(100));
                attempts += 1;
                if attempts >= MAX_ATTEMPTS {
                    return Err(AppError::Ipc {
                        message: "Pipe is busy, max attempts reached".to_string(),
                    });
                }
                continue;
            } else {
                return Err(AppError::Ipc {
                    message: format!("Failed to connect to pipe: error code {}", error),
                });
            }
        }
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), AppError> {
        let mut bytes_written = 0u32;
        let result = unsafe {
            WriteFile(
                self.handle,
                data.as_ptr() as *const winapi::ctypes::c_void,
                data.len() as u32,
                &mut bytes_written,
                ptr::null_mut(),
            )
        };

        if result == 0 {
            return Err(get_last_windows_error());
        }

        if bytes_written != data.len() as u32 {
            return Err(AppError::Ipc {
                message: "Not all data was written to pipe".to_string(),
            });
        }

        unsafe { FlushFileBuffers(self.handle) };
        Ok(())
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, AppError> {
        let mut bytes_read = 0u32;
        let result = unsafe {
            ReadFile(
                self.handle,
                buffer.as_mut_ptr() as *mut winapi::ctypes::c_void,
                buffer.len() as u32,
                &mut bytes_read,
                ptr::null_mut(),
            )
        };

        if result == 0 {
            return Err(get_last_windows_error());
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
