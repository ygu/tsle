use axum::Json;

#[derive(serde::Deserialize)]
pub struct PlayMoveRequest {
  pub game_id: String,
  pub player_id: String,
  pub from: String,
  pub to: String
}

#[derive(serde::Serialize)]
pub struct PlayMoveResponse {
  pub game_id: String,
  pub new_state: String
}

pub async fn play_move(Json(input): Json<PlayMoveRequest>) -> Json<PlayMoveResponse> {
  let game_id = input.game_id;
  Json(PlayMoveResponse {
    game_id: game_id,
    new_state: format!("Move played: {} → {}", input.from, input.to),
  })
}
