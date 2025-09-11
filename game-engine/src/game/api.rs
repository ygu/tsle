use axum::{Router, routing::post};
use std::sync::Arc;
use crate::{app_state::AppState, game::operation::create_game};

pub fn router() -> Router<Arc<AppState>> {
  Router::new()
    .route("/create-game", post(create_game))
}
