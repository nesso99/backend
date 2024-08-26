use sqlx::PgPool;

use super::{CreateUserRequest, UpdateUserRequest, UserModel};

pub async fn create_user(pool: &PgPool, body: CreateUserRequest) -> Result<UserModel, sqlx::Error> {
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

pub async fn update_user(pool: &PgPool, body: UpdateUserRequest) -> Result<UserModel, sqlx::Error> {
    let old_user = find_user_by_id(pool, body.id).await?;
    sqlx::query_as!(
        UserModel,
        r#"UPDATE "user" SET email = $1, username = $2 WHERE id = $3 RETURNING *"#,
        body.email.unwrap_or(old_user.email),
        body.username.unwrap_or(old_user.username),
        body.id
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
