use axum::{Router, routing::post};
use crate::game::operation::create_game;

pub fn router() -> Router {
  Router::new()
    .route("/create-game", post(create_game))
}
