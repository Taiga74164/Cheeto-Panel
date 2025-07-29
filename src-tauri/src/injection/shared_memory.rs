use crate::error::AppError;
use crate::utils::windows_utils::{create_cstring, get_last_windows_error};
use std::ptr;
use winapi::um::{
    handleapi::{CloseHandle, INVALID_HANDLE_VALUE},
    memoryapi::{MapViewOfFile, UnmapViewOfFile, FILE_MAP_WRITE},
    winbase::{CreateFileMappingA},
    winnt::{HANDLE, PAGE_READWRITE},
};

pub struct SharedMemoryFlag {
    map_handle: HANDLE,
    view_ptr: *mut u8,
}

impl SharedMemoryFlag {
    const MEMORY_NAME: &'static str = "UsePipeModeFlag_";

    pub fn new(use_pipe_mode: bool, process_id: u32) -> Result<Self, AppError> {
        let name_cstring = create_cstring(&format!("{}{}", Self::MEMORY_NAME, process_id))?;

        let map_handle = unsafe {
            CreateFileMappingA(
                INVALID_HANDLE_VALUE,
                ptr::null_mut(),
                PAGE_READWRITE,
                0,
                1,
                name_cstring.as_ptr(),
            )
        };

        if map_handle.is_null() {
            return Err(AppError::Process {
                message: format!(
                    "Failed to create file mapping: {}",
                    get_last_windows_error()
                ),
            });
        }

        let view_ptr = unsafe { MapViewOfFile(map_handle, FILE_MAP_WRITE, 0, 0, 1) };

        if view_ptr.is_null() {
            unsafe {
                CloseHandle(map_handle);
            }
            return Err(AppError::Process {
                message: format!("Failed to map view of file: {}", get_last_windows_error()),
            });
        }

        unsafe {
            // *static_cast<char*>(ctx.pView) = usePipeMode ? 1 : 0;
            *(view_ptr as *mut u8) = if use_pipe_mode { 1 } else { 0 };
        }

        Ok(SharedMemoryFlag {
            map_handle: map_handle,
            view_ptr: view_ptr as *mut u8,
        })
    }

    pub fn set_flag(&mut self, use_pipe_mode: bool) {
        unsafe {
            *self.view_ptr = if use_pipe_mode { 1 } else { 0 };
        }
    }

    pub fn get_flag(&self) -> bool {
        if self.view_ptr.is_null() {
            return false;
        }

        unsafe { *self.view_ptr != 0 }
    }

    pub fn release_flag(&mut self) {
        if !self.view_ptr.is_null() {
            unsafe {
                UnmapViewOfFile(self.view_ptr as *mut _);
            }

            self.view_ptr = std::ptr::null_mut();
        }
        
        if !self.map_handle.is_null() || self.map_handle != INVALID_HANDLE_VALUE{
            unsafe {
                CloseHandle(self.map_handle);
            }

            self.map_handle = std::ptr::null_mut();
        }
    }
}

impl Drop for SharedMemoryFlag {
    fn drop(&mut self) {
        self.release_flag();
    }
}
