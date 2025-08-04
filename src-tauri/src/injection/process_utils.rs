use std::ptr::null_mut;

use crate::utils::windows_utils::{create_cstring, get_last_windows_error};
use crate::error::AppError;
use serde::Serialize;
use winapi::shared::winerror::WAIT_TIMEOUT;
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::tlhelp32::TH32CS_SNAPMODULE;
use winapi::um::winbase::INFINITE;
use winapi::{
    shared::minwindef::HMODULE,
    um::{
        handleapi::{CloseHandle, INVALID_HANDLE_VALUE},
        tlhelp32::{
            CreateToolhelp32Snapshot, Module32First, Module32Next, Process32First, Process32Next,
            MODULEENTRY32, PROCESSENTRY32, TH32CS_SNAPPROCESS,
        },
        winnt::HANDLE,
    },
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

    pub fn get_module_handle_in_process(pid: u32, module_name: &str) -> Result<HMODULE, AppError> {
        let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, pid) };
        if snapshot == INVALID_HANDLE_VALUE {
            return Err(AppError::Process {
                message: format!(
                    "Failed to create module snapshot for PID {}: {}",
                    pid,
                    get_last_windows_error()
                ),
            });
        }

        let mut me32: MODULEENTRY32 = unsafe { std::mem::zeroed() };
        me32.dwSize = std::mem::size_of::<MODULEENTRY32>() as u32;

        if unsafe { Module32First(snapshot, &mut me32) } == 0 {
            unsafe { CloseHandle(snapshot) };
            return Err(AppError::Process {
                message: format!(
                    "Failed to create module snapshot for PID {}: {}",
                    pid,
                    get_last_windows_error()
                ),
            });
        }

        loop {
            let current_module = unsafe { std::ffi::CStr::from_ptr(me32.szModule.as_ptr()) }
                .to_str()
                .unwrap_or("");
            if current_module.eq_ignore_ascii_case(module_name) {
                unsafe { CloseHandle(snapshot) };
                return Ok(me32.hModule);
            }

            if unsafe { Module32Next(snapshot, &mut me32) } == 0 {
                break;
            }
        }

        unsafe { CloseHandle(snapshot) };
        Err(AppError::Process {
            message: format!(
                "Module '{}' not found in process with PID {}",
                module_name, pid
            ),
        })
    }

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

    pub fn unload_dll(pid: u32, module_name: &str) -> Result<(), AppError> {
        use winapi::um::libloaderapi::GetProcAddress;

        let h_module = Self::get_module_handle_in_process(pid, module_name)?;

        let process_handle = Self::open_process(
            pid,
            winapi::um::winnt::PROCESS_CREATE_THREAD
                | winapi::um::winnt::PROCESS_QUERY_INFORMATION
                | winapi::um::winnt::PROCESS_VM_OPERATION
                | winapi::um::winnt::PROCESS_VM_WRITE
                | winapi::um::winnt::PROCESS_VM_READ,
        )?;

        let kernel32_name = create_cstring("kernel32.dll")?;
        let h_kernel32 = unsafe { GetModuleHandleA(kernel32_name.as_ptr()) };
        if h_kernel32.is_null() {
            unsafe { CloseHandle(process_handle) };
            return Err(AppError::Process {
                message: "Failed to get handle to kernel32.dll".to_string(),
            });
        }

        let free_library_name = create_cstring("FreeLibrary")?;
        let free_library_addr =
            unsafe { GetProcAddress(h_kernel32, free_library_name.as_ptr()) };
        if free_library_addr.is_null() {
            unsafe { CloseHandle(process_handle) };
            return Err(AppError::Process {
                message: "Failed to get address of FreeLibrary".to_string(),
            });
        }

        let remote_thread = unsafe {
            winapi::um::processthreadsapi::CreateRemoteThread(
                process_handle,
                null_mut(),
                0,
                Some(std::mem::transmute(free_library_addr)),
                h_module as *mut _,
                0,
                null_mut(),
            )
        };

        if remote_thread.is_null() {
            unsafe { CloseHandle(process_handle) };
            return Err(AppError::Process {
                message: format!(
                    "Failed to create remote thread: {}",
                    get_last_windows_error()
                ),
            });
        }

        let wait_result = unsafe { WaitForSingleObject(remote_thread, INFINITE) };
        unsafe {
            CloseHandle(remote_thread);
            CloseHandle(process_handle);
        }

        match wait_result {
            winapi::um::winbase::WAIT_OBJECT_0 => Ok(()),
            WAIT_TIMEOUT => Err(AppError::Process {
                message: "Remote FreeLibrary thread timed out".to_string(),
            }),
            _ => Err(AppError::Process {
                message: format!(
                    "Remote FreeLibrary thread failed: {}",
                    get_last_windows_error()
                ),
            }),
        }
    }
}

#[derive(Serialize)]
pub struct ProcessInfo {
    pub name: String,
    pub pid: u32,
    pub is_running: bool,
}
