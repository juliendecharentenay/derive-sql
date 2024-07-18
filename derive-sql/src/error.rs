pub type Result<T> = std::result::Result<T, Error>;
pub type DeriveSqlResult<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("Not implemented yet")]
  NotImplemented,
  #[error(transparent)]
  FromChronoParseError(#[from] chrono::ParseError),
  #[error(transparent)]
  FromUtf8Error(#[from] std::string::FromUtf8Error),
  #[error(transparent)]
  TryFromIntError(#[from] std::num::TryFromIntError),
  #[error("Conversion of SQL value from `{1}` to type `{0}` is invalid")]
  InvalidTypeForFrom(String, String),
  #[error("Conversion of SQL value to type `{0}` is invalid")]
  InvalidTypeFor(String),
  #[error("The maximum number of parameter - `{0}` - has been exceeded. Requested: `{1}`")]
  SqliteMaximumNumberOfParametersExceeded(usize, usize),
  #[error("Row item `{0}` not found")]
  RowItemNotFound(usize),
  #[error("Object insertion failed")]
  InsertionFail,
  #[error("Unable to convert result to type `{0}`")]
  ResultConversionFail(String),
  #[error("Query returned no result")]
  QueryReturnNoResult,
  #[error("No MySql connection provided to Log proxy")]
  MySqlProxyNoConnectionProvided,
  #[error("No SQLite connection provided to Log proxy")]
  SqliteProxyNoConnectionProvided,
  #[cfg(feature = "mysql")]
  #[error(transparent)]
  MysqlFromValueError(#[from] mysql::FromValueError),
  #[cfg(feature = "mysql")]
  #[error(transparent)]
  MysqlError(#[from] mysql::Error),
  #[cfg(feature = "sqlite")]
  #[error(transparent)]
  RusqliteError(#[from] rusqlite::Error),
  #[error("Error: {0}")]
  Misc(String),
}

impl std::convert::From<&str> for Error {
  fn from(v: &str) -> Self {
    Error::Misc(v.to_string())
  }
}
