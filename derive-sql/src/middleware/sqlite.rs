use super::*;

mod conn; pub use conn::Conn;
mod log; pub use log::Log;

/// Generic trait exposing methods used for interaction with SQLite
/// in `DeriveSqlite` macro implementation.
pub trait SqliteTrait {
  fn execute<P>(&self, sql: &str, params: P) -> rusqlite::Result<usize>
  where P: rusqlite::Params;

  fn query_first<T, P, F>(&self, sql: &str, params: P, f: F) -> rusqlite::Result<T>
  where P: rusqlite::Params,
        F: FnOnce(&rusqlite::Row<'_>) -> rusqlite::Result<T>;

  fn query_map<T, P, F>(&self, sql: &str, params: P, f: F) -> rusqlite::Result<Vec<T>>
  where P: rusqlite::Params,
        F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>;

}
