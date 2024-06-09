//! SQLite connection - a trait is defined to be implemented by SQLite connection.
//! This allow functionalities to be added to the connection directly. Two implementations
//! of the trait are provided: `sqlite::Conn` provide a wrapper around the raw `rusqlite::Conn`;
//! `sqlite::Log` augments a connection by log the requests.
use super::*;

mod conn; pub use conn::Conn;
mod log; pub use log::Log;

/// Generic trait exposing methods used for interaction with SQLite
/// in `DeriveSqlite` macro implementation.
pub trait SqliteTrait {
  fn execute<P>(&self, sql: &str, params: P) -> DeriveSqlResult<usize>
  where P: rusqlite::Params;

  fn query_first<T, P, F>(&self, sql: &str, params: P, f: F) -> DeriveSqlResult<T>
  where P: rusqlite::Params,
        F: FnOnce(&rusqlite::Row<'_>) -> rusqlite::Result<T>;

  fn query_map<T, P, F>(&self, sql: &str, params: P, f: F) -> DeriveSqlResult<Vec<T>>
  where P: rusqlite::Params,
        F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>;

}

/// Auto-implement `Selectable` trait for struct implementing `SqliteTrait`
impl<T, S, I> traits::Select<T, I> for S
where S: SqliteTrait,
      T: traits::AsStatement<I> + traits::IsSelect,
      I: for<'a> std::convert::TryFrom<&'a rusqlite::Row<'a>, Error = rusqlite::Error>,
{
  fn select(&self, s: &T) -> DeriveSqlResult<Vec<I>> {
    self.query_map(s.as_statement()?.as_str(), [], |r| r.try_into())
  }
}

/// Auto-implement `CreateTable` trait for struct implementing `SqliteTrait`
impl<T, S> traits::CreateTable<T> for S
where S: SqliteTrait,
      T: traits::AsStatement<()>
{
  fn create_table(&mut self, s: &T) -> DeriveSqlResult<()> {
    self.execute(s.as_statement()?.as_str(), ())?;
    Ok(())
  }
}
