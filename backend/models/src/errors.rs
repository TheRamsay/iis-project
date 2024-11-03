use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::DbErr;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid request: {0:?}")]
    ValidationError(#[from] validator::ValidationErrors),
    #[error("SQL failed: {0:?}")]
    DatabaseError(#[from] DbErr),
    #[error("Any error: {0:?}")]
    Anyhow(#[from] anyhow::Error),
    #[error("Entity {} not found", .0)]
    NotFound(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Conflict: {0}")]
    Conflict(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        println!("Error: {:?}", self);
        match self {
            Self::ValidationError(err) => (StatusCode::BAD_REQUEST, Json(json!({ "error": err }))),
            Self::DatabaseError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": err.to_string() })),
            ),
            Self::Anyhow(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": err.to_string() })),
            ),
            Self::NotFound(entity) => (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": format!("Entity {} not found", entity) })),
            ),
            Self::Unauthorized(err) => (StatusCode::UNAUTHORIZED, Json(json!({ "error": err }))),
            Self::Conflict(err) => (StatusCode::CONFLICT, Json(json!({ "error": err }))),
        }
        .into_response()
    }
}

pub type AppResult<T> = std::result::Result<T, AppError>;
