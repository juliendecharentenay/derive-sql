use super::*;

mod conn; pub use conn::Conn;
mod log; pub use log::Log;

/// Generic trait exposing methods used for interaction with SQLite
/// in `DeriveMysql` macro implementation.
pub trait MysqlTrait {
  fn query_drop<Q>(&mut self, query: Q) -> DeriveSqlResult<()>
  where Q: AsRef<str>;

  fn query_first<T, Q>(&mut self, query: Q) -> DeriveSqlResult<Option<T>>
  where Q: AsRef<str>,
        T: ::mysql::prelude::FromRow;

  fn query_map<T, F, Q, U>(&mut self, query: Q, f: F) -> DeriveSqlResult<Vec<U>>
  where Q: AsRef<str>,
        T: ::mysql::prelude::FromRow,
        F: FnMut(T) -> U;

  fn exec_drop<Q, P>(&mut self, query: Q, params: P) -> DeriveSqlResult<()>
  where Q: AsRef<str>,
        P: Into<::mysql::Params>;
}

