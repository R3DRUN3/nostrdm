use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    Generic(#[from] anyhow::Error),
}
pub type AppResult<T> = Result<T, AppError>;
