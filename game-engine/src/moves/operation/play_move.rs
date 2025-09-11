use axum::Json;
use axum::http::StatusCode;
use sea_orm::*;

use crate::app_state::ctx;
use crate::http::errors::map_db_err;
use crate::game::viewpoint::Rest;
use crate::game::viewpoint::base::{Entity, ActiveModel, Column};
use crate::game::model::{GameId, GameState};

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

    let db = &ctx().db;
    let rest = Rest::new(db.clone());

    let game = rest.games()
      .filter(Column::Id.eq(game_id.to_string()))
      .one(db)
      .await
      .map_err(map_db_err)?
      .ok_or_else(|| {
        (
          StatusCode::NOT_FOUND,
          format!("Game {} not found", input.game_id),
        )
      })?;

    let state = GameState::from_fen(&game.state)
      .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    // TODO: appliquer réellement le coup
    // state.apply_move(&input.from, &input.to);

    let new_state = state.to_fen();

    Entity::update(ActiveModel {
      id: Set(game.id.clone()),
      state: Set(new_state.clone()),
      ..Default::default()
    })
    .exec(db)
    .await
    .map_err(map_db_err)?;

    Ok(Json(PlayMoveResponse {
      game_id: input.game_id,
      new_state,
    }))
}
