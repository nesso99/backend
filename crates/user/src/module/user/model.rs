use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserModel {
    pub id: Option<i64>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}
