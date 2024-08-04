use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}
