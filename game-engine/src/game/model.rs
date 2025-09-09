use uuid::Uuid;
use chess::Board;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameId(pub Uuid);

impl GameId {
  pub fn new() -> Self {
    Self(Uuid::new_v4())
  }

  pub fn from_str(s: &str) -> Result<GameId, uuid::Error> {
    Uuid::parse_str(s).map(Self)
  }

  pub fn to_string(&self) -> String {
    self.0.to_string()
  }
}

#[derive(Debug, Clone)]
pub struct GameState {
  pub board: Board
}

impl GameState {
  pub fn new() -> Self {
    Self {  board: Board::default() }
  }

  pub fn from_fen(fen: &str) -> Result<Self, String> {
    fen.parse::<Board>()
      .map(|board| GameState { board })
      .map_err(|_| "invalid FEN".to_string())
  }

  pub fn to_fen(&self) -> String {
    self.board.to_string()
  }
}
