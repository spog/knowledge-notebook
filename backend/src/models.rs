//*** Begin File: backend/src/models.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PublicUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: String, // formatted for API
}

impl From<User> for PublicUser {
    fn from(user: User) -> Self {
        PublicUser {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: crate::utils::format_datetime_pretty(user.created_at),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

// JWT claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id as uuid string
    pub exp: usize,
}

//*** End File
