use super::base::Base;
use super::base::Entity;
use sea_orm::*;

pub struct Casing {
  base: Base,
}

impl Casing {
  pub fn new(db: DatabaseConnection) -> Self {
    let base = Base::new(db);
    Self { base }
  }

  pub fn games(&self) -> Select<Entity> {
    self.base.games()
  }
}