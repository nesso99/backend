use alloy::primitives::{keccak256, Address};
use axum::{extract::State, response::IntoResponse, routing::post, Router};
use secp256k1::{rand, PublicKey, Secp256k1, SecretKey};
use sqlx::PgPool;

use crate::{error::AppError, state::AppState};

pub struct WalletRouter;
impl WalletRouter {
    pub fn new_router() -> Router<AppState> {
        Router::new().route("/", post(create_wallets))
    }
}

async fn create_wallets(State(_pool): State<PgPool>) -> Result<impl IntoResponse, AppError> {
    let context = Secp256k1::new();
    let secret_key = SecretKey::new(&mut rand::thread_rng());
    let public_key = PublicKey::from_secret_key(&context, &secret_key);
    let hash = keccak256(&public_key.serialize_uncompressed()[1..]);
    let address = Address::from_slice(&hash[12..]);

    Ok(address.to_string())
}
