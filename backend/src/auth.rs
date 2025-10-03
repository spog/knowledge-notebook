//*** Begin File: backend/src/auth.rs
use crate::models::{Claims};
use anyhow::Context;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, TokenData};
use chrono::{Utc, Duration};
use std::env;

/// Issue a JWT for given subject (user id as string)
pub fn issue_jwt(sub: &str) -> Result<String, anyhow::Error> {
    // Secret should come from env var JWT_SECRET
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "insecure_dev_secret".to_string());
    let exp = (Utc::now() + Duration::hours(24)).timestamp() as usize;
    let claims = Claims {
        sub: sub.to_string(),
        exp,
    };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .context("failed to encode jwt")?;
    Ok(token)
}

/// Validate a JWT and return claims if valid
pub fn validate_jwt(token: &str) -> Result<TokenData<Claims>, anyhow::Error> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "insecure_dev_secret".to_string());
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .context("failed to decode jwt")?;
    Ok(token_data)
}

//*** End File
