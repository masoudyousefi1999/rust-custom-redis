#[derive(Debug)]
pub enum AppError {
    #[allow(unused)]
    Error(String),
}

pub type AppResult<T> = Result<T, AppError>;

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::Error(value.to_string())
    }
}
