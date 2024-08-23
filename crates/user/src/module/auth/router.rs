use std::{
    fmt::Display,
    time::{Duration, SystemTime},
};

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
    response::IntoResponse,
    routing::{get, post},
    Json, RequestPartsExt, Router,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::{config::APP_CONFIG, error::AppError, state::AppState};

use super::LoginResponse;

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = &APP_CONFIG.jwt_secret;
    Keys::new(secret.as_bytes())
});

pub struct AuthRouter;

impl AuthRouter {
    pub fn new_router() -> Router<AppState> {
        Router::new()
            .route("/login", post(login))
            .route("/me", get(me))
    }
}

async fn login() -> Result<impl IntoResponse, AppError> {
    let now = SystemTime::now();
    let expiry = now
        .checked_add(Duration::from_secs(APP_CONFIG.jwt_ttl_secs))
        .expect("Time overflow");
    let expiry = expiry
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize;

    let claims = Claims {
        sub: "b@b.com".to_owned(),
        exp: expiry,
    };
    let token = encode(&Header::default(), &claims, &KEYS.encoding)?;
    Ok(Json(LoginResponse {
        access_token: token,
    }))
}

async fn me(claims: Claims) -> Result<String, AppError> {
    // Send the protected data to the user
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{claims}",
    ))
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Email: {}", self.sub)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await?;
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())?;
        Ok(token_data.claims)
    }
}

#[allow(dead_code)]
struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
