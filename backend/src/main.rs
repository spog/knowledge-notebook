use axum::{
    routing::{get, post},   // added get and post here
    Router,
    extract::State,
    Json,
};
use sqlx::Pool;
use sqlx::Postgres;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tracing_subscriber;
use serde::Deserialize;
//use sqlx::PgPool;

use serde::Serialize;
use sqlx::FromRow;

use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, FromRow)]
struct User {
    id: Uuid,
    username: String,
    email: String,
    #[serde(skip_serializing)]
    password_hash: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
struct PublicUser {
    id: Uuid,
    username: String,
    email: String,
    created_at: DateTime<Utc>,
}

// convert from User → PublicUser
impl From<User> for PublicUser {
    fn from(user: User) -> Self {
        PublicUser {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    username: String,
    email: String,
    password: String,
}

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("Database error: {0}")]
    Db(#[from] sqlx::Error),

    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use password_hash::{SaltString};
use rand_core::OsRng;

fn hash_password(password: &str) -> Result<String, anyhow::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!(e))?  // <-- explicit conversion
        .to_string();
    Ok(hash)
}

fn verify_password(stored_hash: &str, password: &str) -> bool {
    let parsed_hash = PasswordHash::new(stored_hash);
    if let Ok(parsed) = parsed_hash {
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok()
    } else {
        false
    }
}

async fn create_user(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<PublicUser>, AppError> {
    let password_hash = hash_password(&payload.password)?;

    let user: User = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email, password_hash, created_at)
         VALUES ($1, $2, $3, NOW())
         RETURNING id, username, email, password_hash, created_at"
    )
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(password_hash)
    .fetch_one(&pool)
    .await?;

    Ok(Json(PublicUser::from(user)))
}

async fn list_users(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Vec<PublicUser>>, AppError> {
    let users: Vec<User> =
        sqlx::query_as::<_, User>("SELECT id, username, email, password_hash, created_at FROM users")
            .fetch_all(&pool)
            .await?;

    Ok(Json(users.into_iter().map(PublicUser::from).collect()))
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    success: bool,
    message: String,
    // later we’ll return a JWT token here
}

async fn login(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    // Fetch user by email
    let user: Option<User> = sqlx::query_as::<_, User>(
        "SELECT id, username, email, password_hash, created_at FROM users WHERE email = $1"
    )
    .bind(&payload.email)
    .fetch_optional(&pool)
    .await?;

    match user {
        Some(user) => {
            if verify_password(&user.password_hash, &payload.password) {
                Ok(Json(LoginResponse {
                    success: true,
                    message: "Login successful".into(),
                }))
            } else {
                Ok(Json(LoginResponse {
                    success: false,
                    message: "Invalid password".into(),
                }))
            }
        }
        None => Ok(Json(LoginResponse {
            success: false,
            message: "User not found".into(),
        })),
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    // Load env vars
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;

    // Connect to Postgres
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Router setup
    //let app = Router::new().route("/healthz", get(|| async { "ok" }));
    let app = Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .route("/users", post(create_user).get(list_users))
        .route("/auth/login", post(login))
        .with_state(pool);

    // Bind listener
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("🚀 Backend running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
