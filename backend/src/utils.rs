//*** Begin File: backend/src/utils.rs
use chrono::{DateTime, Utc};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use password_hash::SaltString;
use password_hash::rand_core::OsRng;
use password_hash::PasswordHash;
use anyhow::Context;

/// Format DateTime<Utc> in a human-readable pretty format
pub fn format_datetime_pretty(dt: DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

/// Hash a password with Argon2 and return the encoded hash string
pub fn hash_password(password: &str) -> Result<String, anyhow::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let pw_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .context("failed to hash password")?
        .to_string();
    Ok(pw_hash)
}

/// Verify a plain password against encoded hash
pub fn verify_password(stored_hash: &str, password: &str) -> bool {
    match PasswordHash::new(stored_hash) {
        Ok(parsed) => Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok(),
        Err(_) => false,
    }
}
//*** End File
