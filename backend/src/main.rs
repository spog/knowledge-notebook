use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tracing_subscriber;

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
        .with_state(pool);

    // Bind listener
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("🚀 Backend running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
