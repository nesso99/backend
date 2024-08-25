use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}
