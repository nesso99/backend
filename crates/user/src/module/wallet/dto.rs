use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CreateWalletsResponse {
    pub address: String,
}
