use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameId {
  pub id: Uuid
}

impl GameId {
  pub fn new() -> Self {
    Self { id: Uuid::new_v4() }
  }

  pub fn to_string(&self) -> String {
    self.id.to_string()
  }
}

#[derive(Debug, Clone)]
pub struct GameState {
  pub state: String
}

impl GameState {
  pub fn new() -> Self {
    Self {  state: "New game started!".to_string() }
  }
}
