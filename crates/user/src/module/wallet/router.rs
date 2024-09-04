use alloy::primitives::{keccak256, Address};
use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use secp256k1::{rand, PublicKey, Secp256k1, SecretKey};
use serde_json::json;
use sqlx::PgPool;

use crate::{error::AppError, json::ValidatedJson, state::AppState};

use super::{create_wallet, CreateWalletRequest};

pub struct WalletRouter;
impl WalletRouter {
    pub fn new_router() -> Router<AppState> {
        Router::new()
            .route("/", post(create_wallets))
            .route("/kafka", post(try_kafka))
    }
}

async fn try_kafka(
    State(pool): State<PgPool>,
    ValidatedJson(body): ValidatedJson<CreateWalletRequest>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(json!({"message": "kafka"})))
}

async fn create_wallets(
    State(pool): State<PgPool>,
    ValidatedJson(body): ValidatedJson<CreateWalletRequest>,
) -> Result<impl IntoResponse, AppError> {
    let context = Secp256k1::new();
    let secret_key = SecretKey::new(&mut rand::thread_rng());
    let public_key = PublicKey::from_secret_key(&context, &secret_key);
    let hash = keccak256(&public_key.serialize_uncompressed()[1..]);
    let address = Address::from_slice(&hash[12..]);

    let wallet = create_wallet(&pool, body.user_id, address.to_string()).await?;
    Ok(Json(wallet))
}
