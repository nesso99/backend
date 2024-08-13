use axum::{response::IntoResponse, routing::get, Json, Router};
use serde_json::json;

pub struct HealthRouter;

impl HealthRouter {
    pub fn new_router() -> Router<sqlx::Pool<sqlx::Postgres>> {
        Router::new().route("/", get(root))
    }
}

async fn root() -> impl IntoResponse {
    Json(json!({}))
}
