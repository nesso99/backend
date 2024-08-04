use std::time::Duration;

use axum::{
    extract::{Path, State},
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        Method, StatusCode,
    },
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use clap::Parser;
use serde_json::json;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tower_http::cors::{Any, CorsLayer};
use user::{config::Config, dto::CreateUserDto, model::User, version::Version};

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
        .route("/api/:version/users", get(get_users))
        .route("/api/:version/users", post(create_user))
        .route("/api/:version/users/:id", get(get_user))
        .with_state(pool)
        .layer(cors_layer);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_user(
    State(pool): State<PgPool>,
    Json(body): Json<CreateUserDto>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        User,
        r#"INSERT INTO "user" (email,username,password) VALUES ($1, $2, $3) RETURNING *"#,
        body.email,
        body.username,
        body.password
    )
    .fetch_one(&pool)
    .await;

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
    let users = sqlx::query_as!(User, r#"SELECT * FROM "user""#)
        .fetch_all(&pool)
        .await
        .map_err(internal_error)?;
    Ok(Json(users))
}

async fn get_user(
    State(pool): State<PgPool>,
    Path((_, id)): Path<(String, String)>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user = sqlx::query_as!(User, r#"SELECT * FROM "user" WHERE id = $1 LIMIT 1"#, id)
        .fetch_one(&pool)
        .await
        .map_err(internal_error)?;
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
