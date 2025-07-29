use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Windows API error: {code}")]
    WindowsApi { code: u32 },

    #[error("Process error: {message}")]
    Process { message: String },

    #[error("IPC error: {message}")]
    Ipc { message: String },

    #[error("Injection error: {message}")]
    Injection { message: String },
}

impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        error.to_string()
    }
}
