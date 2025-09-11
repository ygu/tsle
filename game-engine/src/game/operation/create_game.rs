use axum::{Json, http::StatusCode};
use sea_orm::*;

use crate::app_state::ctx;
use crate::http::errors::map_db_err;
use crate::game::viewpoint::Rest;
use crate::game::viewpoint::base::{Entity, ActiveModel, Column};
use crate::game::model::{GameId, GameState};

#[derive(serde::Serialize)]
pub struct CreateGameResponse {
  pub game_id: String,
  pub initial_state: String,
}

async fn exists(game_id: &str) -> Result<bool, DbErr> {
  let db = &ctx().db;
  let rest = Rest::new(db.clone());

  let existing_game = rest.games()
    .filter(Column::Id.eq(game_id))
    .one(db)
    .await?;

  Ok(existing_game.is_some())
}

pub async fn create_game() -> Result<Json<CreateGameResponse>, (StatusCode, String)> {
  let db = &ctx().db;

  let game_id = GameId::new();
  let state = GameState::new();

  if exists(&game_id.to_string()).await.map_err(map_db_err)? {
    return Err((StatusCode::CONFLICT, "Game already exists".to_string()));
  }

  Entity::insert(ActiveModel {
    id: Set(game_id.to_string()),
    state: Set(state.to_fen()),
    ..Default::default()
  })
  .exec(db)
  .await
  .map_err(map_db_err)?;

  Ok(Json(CreateGameResponse {
    game_id: game_id.to_string(),
    initial_state: state.to_fen()
  }))
}
