use axum::extract::FromRef;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
    pub redis_pool: bb8::Pool<bb8_redis::RedisConnectionManager>,
}

impl FromRef<AppState> for sqlx::PgPool {
    fn from_ref(app_state: &AppState) -> sqlx::PgPool {
        app_state.db_pool.clone()
    }
}

impl FromRef<AppState> for bb8::Pool<bb8_redis::RedisConnectionManager> {
    fn from_ref(app_state: &AppState) -> bb8::Pool<bb8_redis::RedisConnectionManager> {
        app_state.redis_pool.clone()
    }
}
