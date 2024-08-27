use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub username: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub email: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
}
