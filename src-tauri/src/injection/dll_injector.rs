use crate::error::AppError;
use crate::utils::windows_utils::{create_cstring, get_last_windows_error};
use std::ffi::CString;
use std::ptr::null_mut;
use winapi::um::{
    handleapi::CloseHandle,
    libloaderapi::{GetModuleHandleA, GetProcAddress},
    memoryapi::{VirtualAllocEx, VirtualFreeEx, WriteProcessMemory},
    processthreadsapi::CreateRemoteThread,
    synchapi::WaitForSingleObject,
    winbase::{INFINITE, WAIT_OBJECT_0},
    winnt::{HANDLE, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE},
};

pub struct DllInjector;

impl DllInjector {
    pub fn inject_dll(process_handle: HANDLE, dll_path: &str) -> Result<(), AppError> {
        let dll_path_cstring = create_cstring(dll_path)?;

        unsafe { Self::inject_dll_unsafe(process_handle, &dll_path_cstring) }
    }

    unsafe fn inject_dll_unsafe(h_proc: HANDLE, dll_path: &CString) -> Result<(), AppError> {
        let kernel32_name = create_cstring("kernel32.dll")?;
        let h_kernel = unsafe { GetModuleHandleA(kernel32_name.as_ptr()) };
        if h_kernel.is_null() {
            return Err(AppError::Injection {
                message: "Failed to get kernel32.dll handle".to_string(),
            });
        }

        let loadlib_name = create_cstring("LoadLibraryA")?;
        let load_lib = unsafe { GetProcAddress(h_kernel, loadlib_name.as_ptr()) };
        if load_lib.is_null() {
            return Err(AppError::Injection {
                message: "Failed to get LoadLibraryA address".to_string(),
            });
        }

        let dll_path_bytes = dll_path.as_bytes_with_nul();
        let alloc_mem = unsafe { VirtualAllocEx(
            h_proc,
            null_mut(),
            dll_path_bytes.len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        ) };

        if alloc_mem.is_null() {
            return Err(get_last_windows_error());
        }

        let write_result = unsafe { WriteProcessMemory(
            h_proc,
            alloc_mem,
            dll_path.as_ptr() as *const _,
            dll_path_bytes.len(),
            null_mut(),
        ) };

        if write_result == 0 {
            unsafe { VirtualFreeEx(h_proc, alloc_mem, 0, MEM_RELEASE) };
            return Err(get_last_windows_error());
        }

        let h_thread = unsafe { CreateRemoteThread(
            h_proc,
            null_mut(),
            0,
            Some(std::mem::transmute(load_lib)),
            alloc_mem,
            0,
            null_mut(),
        ) };

        if h_thread.is_null() {
            unsafe { VirtualFreeEx(h_proc, alloc_mem, 0, MEM_RELEASE) };
            return Err(get_last_windows_error());
        }

        let wait_result = unsafe { WaitForSingleObject(h_thread, INFINITE) };

        if wait_result != WAIT_OBJECT_0 {
            unsafe { VirtualFreeEx(h_proc, alloc_mem, 0, MEM_RELEASE) };
            unsafe { CloseHandle(h_thread) };
            return Err(get_last_windows_error());
        }

        unsafe { VirtualFreeEx(h_proc, alloc_mem, 0, MEM_RELEASE) };
        unsafe { CloseHandle(h_thread) };

        Ok(())
    }
}
