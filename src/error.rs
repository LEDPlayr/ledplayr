use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration Error: {0}")]
    ConfigError(String),
    #[error("Storage Error: {0}")]
    StorageError(String),
}
