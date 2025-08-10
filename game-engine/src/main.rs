mod routes;
mod state;

use axum::{Router, routing::get};
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Logs
    tracing_subscriber::fmt::init();

    // Routes
    let app = Router::new()
        .route("/", get(root))
        .merge(routes::routes());

    // Bind
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Server running on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Game Engine API"
}
