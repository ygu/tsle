use std::net::SocketAddr;
use tracing_subscriber;

use game_engine::build_app;

#[tokio::main]
async fn main() {
  // Logs
  tracing_subscriber::fmt::init();

  // Routes
  let app = build_app();

  // Bind
  let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
  tracing::info!("Server running on {}", addr);
  let listener = tokio::net::TcpListener::bind(&addr)
    .await
    .unwrap();
  axum::serve(listener, app).await.unwrap();
}
