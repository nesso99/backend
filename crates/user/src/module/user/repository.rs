use sqlx::PgPool;

use super::{CreateUserDto, UserModel};

pub async fn create_user(pool: &PgPool, body: CreateUserDto) -> Result<UserModel, sqlx::Error> {
    sqlx::query_as!(
        UserModel,
        r#"INSERT INTO "user" (email,username,password) VALUES ($1, $2, $3) RETURNING *"#,
        body.email,
        body.username,
        body.password
    )
    .fetch_one(pool)
    .await
}

pub async fn find_users(pool: &PgPool) -> Result<Vec<UserModel>, sqlx::Error> {
    sqlx::query_as!(UserModel, r#"SELECT * FROM "user""#)
        .fetch_all(pool)
        .await
}

pub async fn find_user_by_id(pool: &PgPool, id: i64) -> Result<UserModel, sqlx::Error> {
    sqlx::query_as!(
        UserModel,
        r#"SELECT * FROM "user" WHERE id = $1 LIMIT 1"#,
        id
    )
    .fetch_one(pool)
    .await
}
