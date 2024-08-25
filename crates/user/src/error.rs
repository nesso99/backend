use std::num::ParseIntError;

use axum::{
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
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            error: String,
            message: String,
        }

        let (status, response) = match self {
            Self::BadRequest => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    error: "Bad Request".to_owned(),
                    message: "Bad Request".to_owned(),
                },
            ),
            Self::ConflictRecord => (
                StatusCode::CONFLICT,
                ErrorResponse {
                    error: "Conflict".to_owned(),
                    message: "Conflict".to_owned(),
                },
            ),
            Self::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    error: "Not Found".to_owned(),
                    message: msg,
                },
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error: "Internal Server Error".to_owned(),
                    message: "Internal Server Error".to_owned(),
                },
            ),
        };

        (status, Json(response)).into_response()
    }
}
