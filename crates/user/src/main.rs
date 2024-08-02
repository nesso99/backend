use std::collections::HashMap;

use axum::{
    async_trait,
    extract::{FromRequestParts, Path},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    RequestPartsExt, Router,
};
use clap::Parser;
use user::config::Config;

#[derive(Debug)]
enum Version {
    V1,
}

#[async_trait]
impl<S> FromRequestParts<S> for Version
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let params: Path<HashMap<String, String>> =
            parts.extract().await.map_err(IntoResponse::into_response)?;

        let version = params
            .get("version")
            .ok_or_else(|| (StatusCode::NOT_FOUND, "version param missing").into_response())?;

        match version.as_str() {
            "v1" => Ok(Version::V1),
            _ => Err((StatusCode::NOT_FOUND, "unknown version").into_response()),
        }
    }
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn handler(version: Version) {
    println!("received request with version {version:?}");
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = Config::parse();
    println!("db {}", config.database_url);

    let app = Router::new()
        .route("/", get(root))
        .route("/api/:version/foo", get(handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
