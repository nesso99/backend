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
            message: String,
        }

        let (status, message) = match self {
            Self::BadRequest => (StatusCode::BAD_REQUEST, "bad request".to_owned()),
            Self::ConflictRecord => (StatusCode::CONFLICT, "conflict record".to_owned()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal error".to_owned(),
            ),
        };

        (status, Json(ErrorResponse { message })).into_response()
    }
}
