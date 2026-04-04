use serde::Serialize;
use specta::Type;

#[derive(Debug, Serialize, Type, Clone)]
#[serde(tag = "code", content = "message")]
pub enum AppError {
    Database(String),
    NotFound(String),
    Validation(String),
    Internal(String),
}

impl AppError {
    pub fn code(&self) -> &'static str {
        match self {
            AppError::Database(_) => "DATABASE",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::Validation(_) => "VALIDATION",
            AppError::Internal(_) => "INTERNAL",
        }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Database(msg) => write!(f, "{}", msg),
            AppError::NotFound(msg) => write!(f, "{}", msg),
            AppError::Validation(msg) => write!(f, "{}", msg),
            AppError::Internal(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<diesel::result::Error> for AppError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => AppError::NotFound("Record not found".into()),
            _ => AppError::Database("A database error occurred".into()),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(_err: std::io::Error) -> Self {
        AppError::Internal("An IO error occurred".into())
    }
}
