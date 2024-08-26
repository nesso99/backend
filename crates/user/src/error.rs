use std::num::ParseIntError;

use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::typed_header::TypedHeaderRejection;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("bad request")]
    BadRequest,
    #[error("conflict record")]
    ConflictRecord,
    #[error("internal error")]
    Internal,
    #[error("not found")]
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
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            error: String,
            message: String,
        }

        let (status, error, message) = match self {
            Self::BadRequest => (
                StatusCode::BAD_REQUEST,
                "Bad Request".to_owned(),
                "Bad Request".to_owned(),
            ),
            Self::ConflictRecord => (
                StatusCode::CONFLICT,
                "Conflict".to_owned(),
                "Conflict".to_owned(),
            ),
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, "Not Found".to_owned(), msg),
            Self::JsonExtractorRejection(rejection) => (
                StatusCode::BAD_REQUEST,
                "Bad Request".to_owned(),
                rejection.body_text(),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_owned(),
                "Internal Server Error".to_owned(),
            ),
        };

        (status, Json(ErrorResponse { error, message })).into_response()
    }
}
