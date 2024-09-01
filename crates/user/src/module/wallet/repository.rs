use sqlx::PgPool;

use crate::error::AppError;

use super::WalletModel;

pub async fn create_wallet(
    pool: &PgPool,
    user_id: i64,
    address: String,
) -> Result<WalletModel, AppError> {
    let result = sqlx::query_as!(
        WalletModel,
        r#"INSERT INTO "wallet" (address,user_id) VALUES ($1, $2) RETURNING *"#,
        address,
        user_id
    )
    .fetch_one(pool)
    .await?;
    Ok(result)
}
