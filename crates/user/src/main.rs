use std::time::Duration;

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        Method, Uri,
    },
    Router,
};
use bb8_redis::RedisConnectionManager;
use clap::Parser;
use redis::AsyncCommands;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};
use user::{
    config::Config,
    error::AppError,
    module::{auth::AuthRouter, health::HealthRouter, user::UserRouter, wallet::WalletRouter},
    state::AppState,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = Config::parse();
    let db_pool = PgPoolOptions::new()
        .max_connections(config.database_max_connection)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database_url)
        .await
        .expect("can't connect to database");

    let manager = RedisConnectionManager::new(config.redis_url).unwrap();
    let redis_pool = bb8::Pool::builder().build(manager).await.unwrap();
    {
        // ping the database before starting
        let mut conn = redis_pool.get().await.unwrap();
        conn.set::<&str, &str, ()>("foo", "bar").await.unwrap();
        let result: String = conn.get("foo").await.unwrap();
        assert_eq!(result, "bar");
    }
    let app_state = AppState {
        db_pool,
        redis_pool,
    };

    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        // .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = Router::new()
        .nest("/api/:version/health", HealthRouter::new_router())
        .nest("/api/:version/auth", AuthRouter::new_router())
        .nest("/api/:version/users", UserRouter::new_router())
        .nest("/api/:version/wallets", WalletRouter::new_router())
        .with_state(app_state)
        .layer(cors_layer);

    // add a fallback service for handling routes to unknown paths
    let app = app.fallback(handler_404);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler_404(method: Method, uri: Uri) -> AppError {
    AppError::NotFound(format!("cannot {} {}", method, uri))
}
