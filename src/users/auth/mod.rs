use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{
    env::{jwt_access_secret, jwt_refresh_secret},
    error::ErrorResponse,
};

use super::model::response::UserSelect;

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    name: String,
    exp: usize,
}

#[derive(Serialize, Deserialize)]
struct WalletClaims {
    sub: String,
    key_store: String,
    exp: usize,
}

enum JwtType {
    Access,
    Refresh,
}

// Encode

fn encode_jwt(user: &UserSelect, token_type: JwtType) -> Result<String, ErrorResponse> {
    let (jwt_secret, duration) = match token_type {
        JwtType::Access => (jwt_access_secret(), Duration::hours(3)),
        JwtType::Refresh => (jwt_refresh_secret(), Duration::hours(720)),
    };

    let exp = Utc::now().checked_add_signed(duration).unwrap().timestamp() as usize;

    let claims = Claims {
        sub: user.pk_user_id.clone(),
        name: user.username.clone(),
        exp,
    };

    let header = Header::new(Algorithm::HS512);

    let key = EncodingKey::from_secret(jwt_secret.as_bytes());

    let encode = encode(&header, &claims, &key)?;

    Ok(encode)
}

pub fn make_access_token(user: &UserSelect) -> Result<String, ErrorResponse> {
    encode_jwt(user, JwtType::Access)
}

pub fn make_refresh_token(user: &UserSelect) -> Result<String, ErrorResponse> {
    encode_jwt(user, JwtType::Refresh)
}

// Decode

fn decode_jwt(jwt: String, token_type: JwtType) -> Result<String, ErrorResponse> {
    let jwt_secret = match token_type {
        JwtType::Access => jwt_access_secret(),
        JwtType::Refresh => jwt_refresh_secret(),
    };

    let key = DecodingKey::from_secret(jwt_secret.as_bytes());

    let validation = Validation::new(Algorithm::HS512);

    let decoded = decode::<Claims>(&jwt, &key, &validation)?;

    Ok(decoded.claims.sub)
}

pub fn decode_access_token(jwt: String) -> Result<String, ErrorResponse> {
    decode_jwt(jwt, JwtType::Access)
}

pub fn decode_refresh_token(jwt: String) -> Result<String, ErrorResponse> {
    decode_jwt(jwt, JwtType::Refresh)
}
