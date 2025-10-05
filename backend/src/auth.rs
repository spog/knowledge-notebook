//*** Begin File: backend/src/auth.rs
use crate::models::{Claims};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
//use serde::Deserialize;
use uuid::Uuid;

use anyhow::Context;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
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

pub struct AuthUser {
    pub user_id: Uuid,
}

/// AuthUser extractor includes a JWT validation
#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header".into()))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or((StatusCode::UNAUTHORIZED, "Invalid Authorization scheme".into()))?;

        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "insecure_dev_secret".into());

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| (StatusCode::UNAUTHORIZED, "auth: Invalid token".into()))?;

        let user_id = Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user id in token".into()))?;

        Ok(AuthUser { user_id })
    }
}

//*** End File
