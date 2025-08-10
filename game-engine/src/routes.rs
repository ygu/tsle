use axum::{Router, routing::post};

use crate::state::{create_game, play_move};

pub fn routes() -> Router {
    Router::new()
        .route("/create-game", post(create_game))
        .route("/play-move", post(play_move))
}
