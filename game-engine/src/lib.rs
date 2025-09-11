pub mod app_state;
pub mod database;
pub mod http;
pub mod game;
pub mod moves;

use axum::{routing::get, Router};
use app_state::AppState;

async fn root() -> &'static str {
  "Game Engine API"
}

pub async fn build_app() -> Router {
  let app_state = AppState::init().await;

  Router::new()
    .route("/", get(root))
    .merge(game::api::router())
    .merge(moves::api::router())
    .with_state(app_state)
}
