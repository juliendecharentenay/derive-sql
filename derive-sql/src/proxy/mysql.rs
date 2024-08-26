use super::*;

#[cfg(feature="compatibility_v0_10")]
mod conn; 
#[cfg(feature="compatibility_v0_10")]
pub use conn::Conn;
#[cfg(feature="compatibility_v0_10")]
mod log; 
#[cfg(feature="compatibility_v0_10")]
pub use log::Log;

#[cfg(feature="compatibility_v0_10")]
/// Generic trait exposing methods used for interaction with SQLite
/// in `DeriveMysql` macro implementation.
pub trait MysqlTrait {
  fn query_drop<Q>(&mut self, query: Q) -> Result<()>
  where Q: AsRef<str>;

  fn query_first<T, Q>(&mut self, query: Q) -> Result<Option<T>>
  where Q: AsRef<str>,
        T: ::mysql::prelude::FromRow;

  fn query_map<T, F, Q, U>(&mut self, query: Q, f: F) -> Result<Vec<U>>
  where Q: AsRef<str>,
        T: ::mysql::prelude::FromRow,
        F: FnMut(T) -> U;

  fn exec_drop<Q, P>(&mut self, query: Q, params: P) -> Result<()>
  where Q: AsRef<str>,
        P: Into<::mysql::Params>;
}

#[cfg(feature="compatibility_v0_10")]
pub struct MysqlProxy<T>
where T: ::mysql::prelude::Queryable,
{
  conn: T
}

#[cfg(feature="compatibility_v0_10")]
impl<T> From<T> for MysqlProxy<T>
where T: ::mysql::prelude::Queryable,
{
  fn from(conn: T) -> Self {
    MysqlProxy { conn }
  }
}

#[cfg(feature="compatibility_v0_10")]
impl<T> traits::ExecuteTrait for MysqlProxy<T>
where T: ::mysql::prelude::Queryable,
{
  fn execute_with_params<P>(&mut self, query: &str, params: P) -> Result<()>
  where P: traits::Params
  {
    let params = params.as_vec_params()?
    .into_iter()
    .map(|p| std::convert::TryInto::<::mysql::Value>::try_into(p))
    .collect::<Result<Vec<::mysql::Value>>>()?;
    self.conn.exec_drop(query, params)?;
    Ok(())
  }
}

pub struct Row {
  row: ::mysql::Row,
}

impl ::mysql::prelude::FromRow for Row {
  fn from_row_opt(row: ::mysql::Row) -> std::result::Result<Row, ::mysql::FromRowError> {
    Ok(Row { row } )
  }
}

impl traits::Row for Row {
  fn get_value(&self, i: usize) -> Option<Result<traits::Value>> { 
    let v: Option<core::result::Result<::mysql::Value, _>> = self.row.get_opt(i);
    if let Some(Ok(v)) = v {
      let v: Result<traits::Value> = v.try_into();
      Some(v)

    } else if let Some(Err(e)) = v {
      Some(Err(e.into()))
      
    } else {
      None
    }
  }
}

/*
struct MyValue {
  value: ::mysql::Value,
}

impl std::convert::From<::mysql::Value> for MyValue {
  fn from(value: ::mysql::Value) -> Self { 
    MyValue { value }
  }
}

impl<T> traits::RefTryInto<T> for MyValue
where T: ::mysql::prelude::FromValue,
{
  fn try_into(&self) -> Result<T> {
    Ok(::mysql::prelude::FromValue::from_value_opt(self.value.clone())?)
  }
}
*/

/*
impl<T> std::convert::TryFrom<&Value> for T
where T: ::mysql::prelude::FromValue
{
  type Error = Error;
  fn try_from(value: &Value) -> Result<Self> {
    Ok(::mysql::prelude::FromValue::from_value_opt(value.value.clone())?)
  }
}
*/

/*
impl traits::Row2<MyValue> for Row
{
  fn get<T>(&self, i: usize) -> Option<Result<T>>
  where MyValue: traits::RefTryInto<T>, // T: for<'a> std::convert::TryFrom<&'a Value, Error=Error>
  {
    let value: Option<::mysql::Value> = self.row.get(i);
    let value: Option<MyValue> = value.map(|v| v.into());
    value.map(|v| traits::RefTryInto::try_into(&v) )
  }
}
*/


#[cfg(feature="compatibility_v0_10")]
impl<T> traits::QueryTrait<Row> for MysqlProxy<T>
where T: ::mysql::prelude::Queryable,
{
  fn query_row(&mut self, query: &str) -> Result<Vec<Row>> {
    Ok( self.conn.query_map(query, |r: Row| r )? )
  }
}

/// Define a trait that provide a transaction. This is required to allow
/// `::mysql::Conn` and `::mysql::PoolConn` to both provide the same
/// `start_transaction` interface.
trait Transaction {
  fn start_transaction(&mut self, tx_opts: ::mysql::TxOpts) -> Result<::mysql::Transaction>;
}

impl Transaction for ::mysql::Conn {
  fn start_transaction(&mut self, tx_opts: ::mysql::TxOpts) -> Result<::mysql::Transaction> {
    Ok(self.start_transaction(tx_opts)?)
  }
}

impl Transaction for ::mysql::PooledConn {
  fn start_transaction(&mut self, tx_opts: ::mysql::TxOpts) -> Result<::mysql::Transaction> {
    Ok(self.start_transaction(tx_opts)?)
  }
}

impl<T> traits::Connection<Row> for T
where T: ::mysql::prelude::Queryable + Transaction,
{
  fn flavor(&self) -> traits::Flavor { traits::Flavor::MySQL }

  fn execute_with_params<S, P>(&mut self, query: S, params: &P) -> Result<()>
  where S: std::convert::AsRef<str>,
        P: traits::Params,
  {
    let params = params.as_vec_params()?
    .into_iter()
    .map(|p| std::convert::TryInto::<::mysql::Value>::try_into(p))
    .collect::<Result<Vec<::mysql::Value>>>()?;
    self.exec_drop(query, params)?;
    Ok(())
  }

  fn execute_with_params_iterator<'a, S, I, P>(&mut self, query: S, params_iter: I) -> Result<()>
  where S: std::convert::AsRef<str>,
        P: traits::Params + 'a,
        I: core::iter::IntoIterator<Item = &'a P>
  {
    use ::mysql::prelude::Queryable;
    let params_arr = params_iter
      .into_iter()
      .map(|params| 
        params.as_vec_params()?.into_iter()
        .map(|p| std::convert::TryInto::<::mysql::Value>::try_into(p))
        .collect::<Result<Vec<::mysql::Value>>>()
      )
      .collect::<Result<Vec<Vec<::mysql::Value>>>>()?;

    let mut conn = self.start_transaction(::mysql::TxOpts::default())?;
    let statement = conn.prep(query)?;
    conn.exec_batch(statement, params_arr.into_iter())?;
    conn.commit()?;
    Ok(())
  }

/*
  fn execute_with_params_rows<S, P>(&mut self, query: S, params: &P) -> Result<Vec<Row>>
  where S: std::convert::AsRef<str>,
        P: traits::Params,
  {
    let params = params.as_vec_params()?
    .into_iter()
    .map(|p| std::convert::TryInto::<::mysql::Value>::try_into(p))
    .collect::<Result<Vec<::mysql::Value>>>()?;
    Ok( self.exec_map(query, params, |r: Row| r)? )
  }
  */

  fn query<S>(&mut self, query: S) -> Result<Vec<Row>>
  where S: std::convert::AsRef<str>
  {
    Ok( self.query_map(query, |r: Row| r)? )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_connection() -> Result<()> {
    let mut conn = ::mysql::Conn::new(
      ::mysql::Opts::from_url("mysql://test@localhost/simpledb").unwrap()
    )?;

    proxy_test::run_connection(&mut conn)?;

    use ::mysql::prelude::Queryable;
    let r: Vec<String> = conn.query_map("SELECT name FROM mytable_proxy_conn", |r: String| r)?;
    assert!(r[0].eq("my name"));

    Ok(())
  }

  #[test]
  fn test_run_with_date() -> Result<()> {
    let mut conn = ::mysql::Conn::new(
      ::mysql::Opts::from_url("mysql://test@localhost/simpledb").unwrap()
    )?;
    proxy_test::run_with_date(&mut conn)?;
    Ok(())
  }
}

