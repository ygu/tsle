use axum::{Router, routing::post};
use crate::moves::operation::play_move;


pub fn router() -> Router {
  Router::new()
    .route("/play-move", post(play_move))
}
