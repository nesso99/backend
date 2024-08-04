use axum::{routing::get, Json, Router};
use clap::Parser;
use serde::Serialize;
use user::{config::Config, version::Version};

#[derive(Serialize)]
pub struct EmptyResponse {}

async fn root() -> Json<EmptyResponse> {
    Json(EmptyResponse {})
}

async fn handler(version: Version) {
    println!("received request with version {version:?}");
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let _config = Config::parse();

    let app = Router::new()
        .route("/", get(root))
        .route("/api/:version/foo", get(handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
