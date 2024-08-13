use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde_json::json;
use sqlx::PgPool;

use crate::error::UserError;

use super::{create_user, find_user_by_id, find_users, CreateUserDto};

pub struct UserRouter;
impl UserRouter {
    pub fn new_router() -> Router<sqlx::Pool<sqlx::Postgres>> {
        Router::new()
            .route("/", get(get_users).post(post_user))
            .route("/:id", get(get_user))
    }
}

async fn post_user(
    State(pool): State<PgPool>,
    Json(body): Json<CreateUserDto>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = create_user(&pool, body).await;

    match query_result {
        Ok(note) => {
            let note_response = json!({"status": "success","data": json!({
                "note": note
            })});
            Ok((StatusCode::CREATED, Json(note_response)))
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "Note with that title already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ))
        }
    }
}

async fn get_users(State(pool): State<PgPool>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let users = find_users(&pool).await.map_err(internal_error)?;
    Ok(Json(users))
}

async fn get_user(
    State(pool): State<PgPool>,
    Path(params): Path<HashMap<String, String>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let id = params
        .get("id")
        .ok_or(UserError::Unknown)
        .map_err(internal_error)?;
    let id = id.parse::<i64>().map_err(internal_error)?;
    let user = find_user_by_id(&pool, id).await.map_err(internal_error)?;
    Ok(Json(user))
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
