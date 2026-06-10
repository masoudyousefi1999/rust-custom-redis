#[derive(Debug)]
pub enum AppError {
    LogicalError(String),
    ServerError(String),
}

pub type AppResult<T> = Result<T, AppError>;

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::ServerError(value.to_string())
    }
}
