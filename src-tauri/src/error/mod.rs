use serde::Serialize;
use specta::Type;

#[derive(Debug, Serialize, Type, Clone)]
#[serde(tag = "code", content = "message")]
pub enum AppError {
    Database(String),
    NotFound(String),
    Validation(String),
    Unauthorized(String),
    Internal(String),
}

impl AppError {
    pub fn code(&self) -> &'static str {
        match self {
            AppError::Database(_) => "DATABASE",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::Validation(_) => "VALIDATION",
            AppError::Unauthorized(_) => "UNAUTHORIZED",
            AppError::Internal(_) => "INTERNAL",
        }
    }

    pub fn note_not_found(id: i32) -> Self {
        AppError::NotFound(format!("Note with id {id} not found"))
    }

    pub fn user_not_found(id: i32) -> Self {
        AppError::NotFound(format!("User with id {id} not found"))
    }

    pub fn validation(msg: impl Into<String>) -> Self {
        AppError::Validation(msg.into())
    }

    pub fn unauthorized(msg: impl Into<String>) -> Self {
        AppError::Unauthorized(msg.into())
    }

    pub fn missing_id(entity: &str) -> Self {
        AppError::Internal(format!("{entity} missing id"))
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Database(msg) => write!(f, "{msg}"),
            AppError::NotFound(msg) => write!(f, "{msg}"),
            AppError::Validation(msg) => write!(f, "{msg}"),
            AppError::Unauthorized(msg) => write!(f, "{msg}"),
            AppError::Internal(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<diesel::result::Error> for AppError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => AppError::NotFound("Record not found".into()),
            _ => AppError::Database(format!("Database error: {err}")),
        }
    }
}

impl From<diesel::result::ConnectionError> for AppError {
    fn from(err: diesel::result::ConnectionError) -> Self {
        AppError::Database(format!("Connection error: {err}"))
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Internal(format!("IO error: {err}"))
    }
}
