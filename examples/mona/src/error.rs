pub type Result<T> = core::result::Result<T, MyError>;

#[derive(thiserror::Error, Debug)]
pub enum MyError {
  #[error(transparent)]
  DeriveSql(#[from] derive_sql::Error),
  #[error(transparent)]
  Boxed(#[from] Box<dyn std::error::Error>),
  #[error(transparent)]
  Rusqlite(#[from] rusqlite::Error),
  #[error("Connection is not initialized. Call /api/initiate first")]
  ConnectionNotInitialized,
  #[error(transparent)]
  ReqwestError(#[from] reqwest::Error),
}
