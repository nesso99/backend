use axum::{response::IntoResponse, routing::get, Json, Router};
use serde_json::json;

use crate::state::AppState;

pub struct AuthRouter;

impl AuthRouter {
    pub fn new_router() -> Router<AppState> {
        Router::new().route("/", get(root))
    }
}

async fn root() -> impl IntoResponse {
    Json(json!({}))
}
