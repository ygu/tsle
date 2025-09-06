use axum::Json;
use crate::game::model::GameState;
use crate::game::model::GameId;

#[derive(serde::Serialize)]
pub struct CreateGameResponse {
  pub game_id: String,
  pub initial_state: String,
}

pub async fn create_game() -> Json<CreateGameResponse> {
  let game_id = GameId::new();
  let state = GameState::new();
  Json(CreateGameResponse {
    game_id: game_id.to_string(),
    initial_state: state.state,
  })
}
