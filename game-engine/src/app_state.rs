use std::sync::Arc;
use once_cell::sync::OnceCell;
use sea_orm::DatabaseConnection;

use crate::database;

#[derive(Clone, Debug)]
pub struct AppState {
  pub db: DatabaseConnection,
}

static APP_STATE: OnceCell<Arc<AppState>> = OnceCell::new();

impl AppState {
  pub async fn init() -> Arc<Self> {
    let db = database::init_database().await;

    let state = Arc::new(AppState { db });
    APP_STATE.set(state.clone()).expect("AppState already initialized");
    state
  }
}

pub fn ctx() -> &'static AppState {
  APP_STATE.get().expect("AppState not initialized")
}
