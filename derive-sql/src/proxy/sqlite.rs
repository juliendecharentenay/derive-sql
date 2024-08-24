//! SQLite connection - a trait is defined to be implemented by SQLite connection.
//! This allow functionalities to be added to the connection directly. Two implementations
//! of the trait are provided: `sqlite::Conn` provide a wrapper around the raw `rusqlite::Conn`;
//! `sqlite::Log` augments a connection by log the requests.
use super::*;

#[cfg(feature = "compatibility_v0_10")]
mod conn; 
#[cfg(feature = "compatibility_v0_10")]
pub use conn::Conn;
#[cfg(feature = "compatibility_v0_10")]
mod log; 
#[cfg(feature = "compatibility_v0_10")]
pub use log::Log;

#[cfg(feature = "compatibility_v0_10")]
/// Generic trait exposing methods used for interaction with SQLite
/// in `DeriveSqlite` macro implementation.
pub trait SqliteTrait {
  fn execute<P>(&self, sql: &str, params: P) -> Result<usize>
  where P: rusqlite::Params;

  fn query_first<T, P, F>(&self, sql: &str, params: P, f: F) -> Result<T>
  where P: rusqlite::Params,
        F: FnOnce(&rusqlite::Row<'_>) -> rusqlite::Result<T>;

  fn query_map<T, P, F>(&self, sql: &str, params: P, f: F) -> Result<Vec<T>>
  where P: rusqlite::Params,
        F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>;

}

#[cfg(feature = "compatibility_v0_10")]
/// Auto-implement `Selectable` trait for struct implementing `SqliteTrait`
impl<T, S, I> traits::Select<T, I> for S
where S: SqliteTrait,
      T: traits::AsStatement<I> + traits::IsSelect,
      I: for<'a> std::convert::TryFrom<&'a rusqlite::Row<'a>, Error = rusqlite::Error>,
{
  fn select(&self, s: &T) -> Result<Vec<I>> {
    self.query_map(s.as_statement()?.as_str(), [], |r| r.try_into())
  }
}

#[cfg(feature = "compatibility_v0_10")]
/// Auto-implement `CreateTable` trait for struct implementing `SqliteTrait`
impl<T, S> traits::CreateTable<T> for S
where S: SqliteTrait,
      T: traits::AsStatement<()>
{
  fn create_table(&mut self, s: &T) -> Result<()> {
    self.execute(s.as_statement()?.as_str(), ())?;
    Ok(())
  }
}

pub struct Row {
  values: Vec<traits::Value>,
}

impl std::convert::TryFrom<(usize, &rusqlite::Row<'_>)> for Row {
  type Error = rusqlite::Error;
  fn try_from((column_count, row): (usize, &rusqlite::Row<'_>)) -> std::result::Result<Self, Self::Error> {
    let mut values = Vec::new();
    for i in 0..column_count {
      let v = match row.get_ref(i)? {
        ::rusqlite::types::ValueRef::Null       => traits::Value::Null,
        ::rusqlite::types::ValueRef::Integer(v) => traits::Value::Integer(v),
        ::rusqlite::types::ValueRef::Real(v)    => traits::Value::Real(v),
        ::rusqlite::types::ValueRef::Text(v)    => traits::Value::Text(std::str::from_utf8(v)?.to_string()),
        ::rusqlite::types::ValueRef::Blob(v)    => traits::Value::Blob(v.to_vec()),
      };
      values.push(v);
    }
    Ok(Row { values })
  }
}

impl traits::Row for Row {
  fn get_value(&self, i: usize) -> Option<Result<traits::Value>> { 
    self.values.get(i).map(|v| Ok(v.clone()))
  }
}

fn execute<'a, P>(statement: &mut rusqlite::Statement<'a>, params: &P) -> Result<()>
where P: traits::Params,
{
    let params: Vec<traits::Param> = params.as_vec_params()?;
    let _ = match params.len() {
      0 => statement.execute(())?,
      1 => statement.execute([ &params[0] ] )?,
      2 => statement.execute([ &params[0], &params[1] ] )?,
      3 => statement.execute([ &params[0], &params[1], &params[2], ] )?,
      4 => statement.execute([ &params[0], &params[1], &params[2], &params[3], ] )?,
      5 => statement.execute([ &params[0], &params[1], &params[2], &params[3], &params[4], ] )?,
      6 => statement.execute([ &params[0], &params[1], &params[2], &params[3], &params[4], &params[5], ] )?,
      7 => statement.execute([ &params[0], &params[1], &params[2], &params[3], &params[4], &params[5], &params[6], ] )?,
      8 => statement.execute([ &params[0], &params[1], &params[2], &params[3], &params[4], &params[5], &params[6], &params[7], ] )?,
      9 => statement.execute([ &params[0], &params[1], &params[2], &params[3], &params[4], &params[5], &params[6], &params[7], &params[8], ] )?,
     10 => statement.execute([ &params[0], &params[1], &params[2], &params[3], &params[4], &params[5], &params[6], &params[7], &params[8], &params[9], ] )?,
     11 => statement.execute([ &params[0], &params[1], &params[2], &params[3], &params[4], &params[5], &params[6], &params[7], &params[8], &params[9], &params[10], ] )?,
     12 => statement.execute([ &params[0], &params[1], &params[2], &params[3], &params[4], &params[5], &params[6], &params[7], &params[8], &params[9], &params[10], &params[11], ] )?,
      // _ => { self.conn.execute(query, params.iter().collect::<Vec<&traits::Param>>().as_slice())?; },
      _ => { return Err(Error::SqliteMaximumNumberOfParametersExceeded(12, params.len())); },
    };
    Ok(())
}

impl traits::Connection<Row> for rusqlite::Connection
{
  fn flavor(&self) -> traits::Flavor { traits::Flavor::SQLite }

  fn execute_with_params<S, P>(&mut self, query: S, params: &P) -> Result<()>
  where S: std::convert::AsRef<str>,
        P: traits::Params,
  {
    let mut statement = self.prepare(query.as_ref())?;
    execute(&mut statement, params)
  }

  fn execute_with_params_iterator<'a, S, I, P>(&mut self, query: S, params_iter: I) -> Result<()>
  where S: std::convert::AsRef<str>,
        P: traits::Params + 'a,
        I: core::iter::IntoIterator<Item = &'a P>
  {
    let tx = self.transaction()?;
    {
      let mut statement = tx.prepare(query.as_ref())?;
      for params in params_iter { execute(&mut statement, params)?; }
    }
    tx.commit()?;
    Ok(())
  }

/*
  fn execute_with_params_rows<S, P>(&mut self, query: S, params: &P) -> Result<Vec<Row>>
  where S: std::convert::AsRef<str>,
        P: traits::Params,
  {
    let mut statement = self.prepare(query.as_ref())?;
    let column_count = statement.column_count();

    let f = |row: &rusqlite::Row<'_>| { Ok((column_count, row).try_into()?) };
    let params: Vec<traits::Param> = params.as_vec_params()?;
    let r = match params.len() {
      0 => statement.query_map((), f)?,
      1 => statement.query_map([ &params[0] ], f )?,
      2 => statement.query_map([ &params[0], &params[1] ], f )?,
      // _ => { self.conn.execute(query, params.iter().collect::<Vec<&traits::Param>>().as_slice())?; },
      _ => { return Err(Error::NotImplemented); },
    }
    .collect::<rusqlite::Result<Vec<Row>>>()?;

    Ok(r)
  }
  */


  fn query<S>(&mut self, query: S) -> Result<Vec<Row>>
  where S: std::convert::AsRef<str>
  {
    use fallible_iterator::FallibleIterator;

    let mut statement = self.prepare(query.as_ref())?;
    let column_count = statement.column_count();

    let r = statement.query(())?
    .map(|row| Ok((column_count, row).try_into()?) )
    .collect::<Vec<Row>>()?;

    Ok(r)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_connection() -> Result<()> {
    let mut conn = rusqlite::Connection::open_in_memory()?;

    proxy_test::run_connection(&mut conn)?;

    let r: Vec<String> = conn.prepare("SELECT name FROM mytable_proxy_conn")?
    .query_map(
      (),
      |r| Ok(r.get(0)?)
    )?.collect::<rusqlite::Result<Vec<String>>>()?;
    assert!(r[0].eq("my name"));
    
    Ok(())
  }

  #[test]
  fn test_run_with_date() -> Result<()> {
    let mut conn = rusqlite::Connection::open_in_memory()?;
    proxy_test::run_with_date(&mut conn)?;
    Ok(())
  }
}
