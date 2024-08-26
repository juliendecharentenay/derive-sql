use super::*;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("MySQL connection parameters hostname and/or user are not specified. Use option -m/--hostname to specify the hostname and -u/--username to specified the username")]
  UnspecifiedMysqlParameters,
  #[error("SQLite filename is not specified. Use option -f/--filename")]
  UnspecifiedSqliteFilename,
  #[error("Action is not specified")]
  UnspecifiedAction,
  #[error("Database type is not specified")]
  UnspecifiedDatabaseType,
  #[error("Database type `{0}` is not supported")]
  UnsupportedDatabaseType(String),
  #[error("Not implemented")]
  NotImplemented,

  // #[error("Connection is not initialized. Call /api/initiate first")]
  // ConnectionNotInitialized,

  #[error(transparent)]
  SerdeJsonError(#[from] serde_json::Error),
  #[error(transparent)]
  StdIoError(#[from] std::io::Error),
  #[error(transparent)]
  MySqlUrlError(#[from] mysql::UrlError),
  #[error(transparent)]
  MySqlError(#[from] mysql::Error),
  #[error(transparent)]
  Rusqlite(#[from] rusqlite::Error),
  #[error(transparent)]
  DeriveSql(#[from] derive_sql::Error),
  #[error(transparent)]
  ReqwestError(#[from] reqwest::Error),

  // #[error(transparent)]
  // Boxed(#[from] Box<dyn std::error::Error>),
}
