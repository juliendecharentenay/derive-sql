pub type DeriveSqlResult<T> = Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("No MySql connection provided to Log proxy")]
  MySqlProxyNoConnectionProvided,
  #[error("No SQLite connection provided to Log proxy")]
  SqliteProxyNoConnectionProvided,
  #[cfg(feature = "mysql")]
  #[error(transparent)]
  MysqlError(#[from] mysql::Error),
  #[cfg(feature = "sqlite")]
  #[error(transparent)]
  RusqliteError(#[from] rusqlite::Error),
  #[error("Error: {0}")]
  Misc(String),
}
