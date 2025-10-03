//*** Begin File: backend/src/main.rs
use axum::{
    routing::{get, post},
    Router, Json,
};
use std::net::SocketAddr;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

mod models;
mod utils;
mod auth;
mod users;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    // Load env vars
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

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
        .route("/users", get(users::list_users))
        .with_state(pool);

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("🚀 Backend running at http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

//*** End File
