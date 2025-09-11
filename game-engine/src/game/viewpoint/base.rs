use sea_orm::*;

pub struct Base {
    pub db: DatabaseConnection,
}

impl Base {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn games(&self) -> Select<Entity> {
        Entity::find()
    }
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "games")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub state: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
