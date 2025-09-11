use sea_orm::{Database, DatabaseConnection};

pub async fn init_database() -> DatabaseConnection {
    let url = std::env::var("DATABASE_URL")
      .expect("DATABASE_URL must be set");
    Database::connect(url)
        .await
        .expect("Database connection failed")
}