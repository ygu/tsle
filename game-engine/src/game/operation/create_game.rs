use axum::Json;

use crate::store::STORE;
use crate::game::model::GameId;
use crate::game::model::GameState;

#[derive(serde::Serialize)]
pub struct CreateGameResponse {
  pub game_id: String,
  pub initial_state: String,
}

pub async fn create_game() -> Json<CreateGameResponse> {
  let mut store = STORE.write().unwrap();
  let game_id = GameId::new();
  let state = GameState::new();
  store.insert(game_id.clone(), state.clone());

  Json(CreateGameResponse {
    game_id: game_id.to_string(),
    initial_state: state.to_fen()
  })
}
