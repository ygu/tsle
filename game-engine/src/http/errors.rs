use axum::http::StatusCode;
use sea_orm::{DbErr, RuntimeErr};
use std::borrow::Cow;

pub fn map_db_err(e: DbErr) -> (StatusCode, String) {
  match e {
    DbErr::RecordNotFound(_) => (StatusCode::NOT_FOUND, "not found".into()),
    DbErr::Conn(_) => (StatusCode::SERVICE_UNAVAILABLE, "db unavailable".into()),
    DbErr::Type(_) | DbErr::Json(_) => (StatusCode::BAD_REQUEST, e.to_string()),
    DbErr::Exec(RuntimeErr::SqlxError(ref err)) |
    DbErr::Query(RuntimeErr::SqlxError(ref err)) => {
      if let Some(dbe) = err.as_database_error() {
        if dbe.code() == Some(Cow::Borrowed("23505")) {
          return (StatusCode::CONFLICT, "duplicate".into());
        }
      }
      (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }
    _ => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
  }
}