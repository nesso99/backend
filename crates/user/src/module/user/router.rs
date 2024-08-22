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

use crate::{error::AppError, state::AppState};

use super::{create_user, find_user_by_id, find_users, CreateUserDto};

pub struct UserRouter;
impl UserRouter {
    pub fn new_router() -> Router<AppState> {
        Router::new()
            .route("/", get(get_users).post(post_user))
            .route("/:id", get(get_user))
    }
}

async fn post_user(
    State(pool): State<PgPool>,
    Json(body): Json<CreateUserDto>,
) -> Result<impl IntoResponse, AppError> {
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
                return Err(AppError::ConflictRecord);
            }
            Err(AppError::Internal)
        }
    }
}

async fn get_users(State(pool): State<PgPool>) -> Result<impl IntoResponse, AppError> {
    let users = find_users(&pool).await?;
    Ok(Json(users))
}

async fn get_user(
    State(pool): State<PgPool>,
    Path(params): Path<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    let id = params.get("id").ok_or(AppError::BadRequest)?;
    let id = id.parse::<i64>()?;
    let user = find_user_by_id(&pool, id).await?;
    Ok(Json(user))
}
