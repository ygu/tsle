pub mod store;
pub mod game;
pub mod moves;

use axum::{routing::get, Router};

async fn root() -> &'static str {
  "Game Engine API"
}

pub fn build_app() -> Router {
  Router::new()
    .route("/", get(root))
    .merge(game::api::router())
    .merge(moves::api::router())
}
