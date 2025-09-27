use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Build router
    let app = Router::new().route("/healthz", get(healthz));

    // Address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Backend running at http://{}", addr);

    // Start server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn healthz() -> &'static str {
    "ok"
}

