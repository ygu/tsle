use axum::{Router, routing::post};
use std::sync::Arc;

use crate::{app_state::AppState, moves::operation::play_move};


pub fn router() -> Router<Arc<AppState>> {
  Router::new()
    .route("/play-move", post(play_move))
}
