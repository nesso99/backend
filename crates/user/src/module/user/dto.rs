use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}
