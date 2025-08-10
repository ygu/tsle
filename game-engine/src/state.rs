use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct MoveInput {
    pub game_id: String,
    pub from: String,
    pub to: String,
}

#[derive(Serialize)]
pub struct GameResponse {
    pub game_id: String,
    pub message: String,
}

pub async fn create_game() -> impl IntoResponse {
    let game_id = Uuid::new_v4();
    Json(GameResponse {
        game_id: game_id.to_string(),
        message: "Game created".to_string(),
    })
}

pub async fn play_move(Json(input): Json<MoveInput>) -> impl IntoResponse {
    Json(GameResponse {
        game_id: input.game_id,
        message: format!("Move played: {} → {}", input.from, input.to),
    })
}
