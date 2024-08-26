use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use sqlx::PgPool;

use crate::{error::AppError, json::Json, state::AppState};

use super::{create_user, find_user_by_id, find_users, CreateUserRequest};

pub struct UserRouter;
impl UserRouter {
    pub fn new_router() -> Router<AppState> {
        Router::new()
            .route("/", get(get_users).post(create_users))
            .route("/:id", get(get_user))
    }
}

async fn create_users(
    State(pool): State<PgPool>,
    Json(body): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let query_result = create_user(&pool, body).await?;
    Ok(Json(query_result))
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
