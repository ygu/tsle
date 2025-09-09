use axum::Json;
use axum::http::StatusCode;

use crate::store::STORE;
use crate::game::model::GameId;

#[derive(serde::Deserialize)]
pub struct PlayMoveRequest {
  pub game_id: String,
  pub from: String,
  pub to: String
}

#[derive(serde::Serialize)]
pub struct PlayMoveResponse {
  pub game_id: String,
  pub new_state: String
}

pub async fn play_move(Json(input): Json<PlayMoveRequest>)
  -> Result<Json<PlayMoveResponse>, (StatusCode, String)> {
    let game_id = GameId::from_str(&input.game_id)
      .map_err(|e| {
        (
          StatusCode::BAD_REQUEST,
          format!("Invalid GameId input: {}", e),
        )
      })?;

    let mut store = STORE.write().unwrap();

    let state = store.get_mut(&game_id)
      .ok_or_else(|| {
        (
          StatusCode::NOT_FOUND,
          format!("Game {} not found", input.game_id),
        )
      })?;

    // state.apply_move(&input.from, &input.to);

    let new_state = state.to_fen();

    Ok(Json(PlayMoveResponse {
      game_id: input.game_id,
      new_state,
    }))
}
