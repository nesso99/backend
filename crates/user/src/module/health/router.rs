use axum::{response::IntoResponse, routing::get, Json, Router};
use serde_json::json;
use yata::helpers::RandomCandles;
use yata::indicators::MACD;
use yata::prelude::*;

use crate::state::AppState;

pub struct HealthRouter;

impl HealthRouter {
    pub fn new_router() -> Router<AppState> {
        Router::new()
            .route("/", get(root))
            .route("/yata", get(yata))
    }
}

async fn root() -> impl IntoResponse {
    Json(json!({}))
}

async fn yata() -> impl IntoResponse {
    let candles: Vec<_> = RandomCandles::new().take(100).collect();
    let mut iter = candles.iter().cycle();
    let mut indicator = MACD::default().init(iter.next().unwrap()).unwrap();
    for _ in 0..50 {
        indicator.next(iter.next().unwrap());
    }
    format!("{:?}", indicator).into_response()
}
