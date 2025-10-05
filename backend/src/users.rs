//*** Begin File: backend/src/users.rs
use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;
use serde_json::json;
use crate::auth::AuthUser;
use crate::models::{CreateUser, PublicUser, User, LoginRequest, LoginResponse};
use crate::utils::{hash_password, verify_password};
use crate::auth::{issue_jwt};

/// Register a new user (POST /auth/register)
pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, String)> {
    // Hash password
    let pw = hash_password(&payload.password).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Insert user, returning full row
    let user: User = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email, password_hash, created_at)
         VALUES ($1, $2, $3, NOW())
         RETURNING id, username, email, password_hash, created_at"
    )
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(&pw)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let public: PublicUser = user.into();
    Ok((StatusCode::CREATED, Json(json!({ "user": public }))))
}

/// Login (POST /auth/login) returns JSON with token
pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let user_opt: Option<User> = sqlx::query_as::<_, User>(
        "SELECT id, username, email, password_hash, created_at FROM users WHERE email = $1"
    )
    .bind(&payload.email)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match user_opt {
        Some(user) => {
            if verify_password(&user.password_hash, &payload.password) {
                let token = issue_jwt(&user.id.to_string()).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                Ok(Json(LoginResponse { token }))
            } else {
                Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))
            }
        }
        None => Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string())),
    }
}

/// Protected route: list users for debugging (GET /users)
pub async fn list_users(
    State(pool): State<PgPool>,
    _auth: AuthUser, // <── extractor used here
) -> Result<Json<Vec<PublicUser>>, (axum::http::StatusCode, String)> {
    let users = sqlx::query_as::<_, User>("SELECT id, username, email, password_hash, created_at FROM users")
        .fetch_all(&pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let public_users = users.into_iter().map(PublicUser::from).collect();
    Ok(Json(public_users))
}

//*** End File
