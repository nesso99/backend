use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct WalletModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub address: String,
    pub user_id: i64,
}
