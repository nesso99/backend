use std::time::Duration;

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    Router,
};
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};
use trading_bot::config::Config;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = Config::parse();
    let pool = PgPoolOptions::new()
        .max_connections(config.database_max_connection)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database_url)
        .await
        .expect("can't connect to database");

    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        // .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = Router::new().with_state(pool).layer(cors_layer);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
