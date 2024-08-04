use std::time::Duration;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use clap::Parser;
use serde_json::json;
use sqlx::{postgres::PgPoolOptions, PgPool};
use user::{config::Config, model::User, version::Version};

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

    let app = Router::new()
        .route("/", get(root))
        .route("/api/:version/foo", get(handler))
        .route("/api/:version/users", get(get_users))
        .with_state(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_users(State(pool): State<PgPool>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let users = sqlx::query_as!(User, r#"SELECT * FROM "user""#)
        .fetch_all(&pool)
        .await
        .map_err(internal_error)?;
    Ok(Json(users))
}

// async fn get_user(State(pool): State<PgPool>) -> Result<String, (StatusCode, String)> {
//     sqlx::query_scalar("select * from \"user\"")
//         .fetch_one(&pool)
//         .await
//         .map_err(internal_error)
// }

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
