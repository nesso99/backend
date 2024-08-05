use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
}
