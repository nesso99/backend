use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct CreateWalletRequest {
    #[validate(range(min = 1))]
    pub user_id: i64,
}
