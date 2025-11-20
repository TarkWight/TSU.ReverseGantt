use axum::{
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Invalid ID format: {0}")]
    InvalidId(#[from] uuid::Error),
}

pub type AppResult<T> = Result<T, AppError>;