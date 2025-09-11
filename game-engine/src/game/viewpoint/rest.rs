use super::casing::Casing;
use super::base::Entity;
use sea_orm::*;

pub struct Rest {
  casing: Casing,
}

impl Rest {
  pub fn new(db: DatabaseConnection) -> Self {
    let casing = Casing::new(db);
    Self { casing }
  }

  pub fn games(&self) -> Select<Entity> {
    self.casing.games()
  }
}