use std::time::Duration;

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use clap::Parser;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};
use user::{config::Config, module::user::new_user_router, version::Version};

async fn root() -> impl IntoResponse {
    Json(json!({}))
}

async fn handler(version: Version) {
    println!("received request with version {version:?}");
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = Config::parse();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database_url)
        .await
        .expect("can't connect to database");

    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        // .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = Router::new()
        .route("/", get(root))
        .route("/api/:version/foo", get(handler))
        .nest("/api/:version/users", new_user_router())
        .with_state(pool)
        .layer(cors_layer);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
