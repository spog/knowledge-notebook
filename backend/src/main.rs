//*** Begin File: backend/src/main.rs
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use tracing::error;

mod models;
mod utils;
mod auth;
mod users;
mod notes;
//use users::{register, login, list_users};
//use notes::{create_note, list_notes};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    // Load env vars
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // Check if JWT_SECRET is set
    if env::var("JWT_SECRET").is_err() {
        error!("Environment variable JWT_SECRET is not set. Refusing to start.");
        std::process::exit(1);
    }

    // Connect to Postgres pool
    let pool: PgPool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Build router and mount routes
    let app = Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .route("/auth/register", post(users::register))
        .route("/auth/login", post(users::login))
        // Protected routes
        .route("/users", get(users::list_users))
        .route("/notes", post(notes::create_note).get(notes::list_notes))
        .with_state(pool);

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("🚀 Backend running at http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

//*** End File
