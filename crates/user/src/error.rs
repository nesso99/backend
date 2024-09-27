use std::num::ParseIntError;

use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::typed_header::TypedHeaderRejection;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("conflict record")]
    ConflictRecord,
    #[error("internal error")]
    Internal,
    #[error("not found: {0}")]
    NotFound(String),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    TypedHeaderRejectionError(#[from] TypedHeaderRejection),
    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
    #[error(transparent)]
    Argo2PasswordHashError(#[from] argon2::password_hash::Error),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::ConflictRecord => (StatusCode::CONFLICT, "Conflict".to_string()),
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            Self::JsonExtractorRejection(rejection) => {
                (StatusCode::BAD_REQUEST, rejection.body_text())
            }
            Self::ValidationError(err) => (
                StatusCode::BAD_REQUEST,
                format!("Input validation error: [{}]", err).replace('\n', ", "),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
        };

        let body = Json(json!({
            "error": status.to_string(),
            "message": error_message,
        }));

        (status, body).into_response()
    }
}
