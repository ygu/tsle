use std::collections::HashMap;
use std::sync::{RwLock, LazyLock};

use crate::game::model::{GameId, GameState};

pub static STORE: LazyLock<RwLock<HashMap<GameId, GameState>>> =
  LazyLock::new(|| RwLock::new(HashMap::new()));
