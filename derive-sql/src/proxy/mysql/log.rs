use super::*;

pub struct Log<C>
where C: MysqlTrait
{
  conn: Option<C>,
}

impl<C> std::default::Default for Log<C>
where C: MysqlTrait
{
  fn default() -> Self { Log { conn: None } }
}

impl<C> Log<C>
where C: MysqlTrait
{
  pub fn with(mut self, conn: C) -> Self { self.conn = Some(conn); self }

  fn conn_mut(&mut self) -> DeriveSqlResult<&mut C> { 
    self.conn.as_mut()
    .ok_or(Error::MySqlProxyNoConnectionProvided)
  }

  fn log(&self, sql: &str) { ::log::info!("{sql}"); }
}

impl<C> MysqlTrait for Log<C>
where C: MysqlTrait
{
  fn query_drop<Q>(&mut self, query: Q) -> DeriveSqlResult<()>
  where Q: AsRef<str>
  {
    self.log(query.as_ref());
    self.conn_mut()?.query_drop(query)
  }

  fn query_first<T, Q>(&mut self, query: Q) -> DeriveSqlResult<Option<T>>
  where Q: AsRef<str>,
        T: ::mysql::prelude::FromRow
  {
    self.log(query.as_ref());
    self.conn_mut()?.query_first(query)
  }

  fn query_map<T, F, Q, U>(&mut self, query: Q, f: F) -> DeriveSqlResult<Vec<U>>
  where Q: AsRef<str>,
        T: ::mysql::prelude::FromRow,
        F: FnMut(T) -> U
  {
    self.log(query.as_ref());
    self.conn_mut()?.query_map(query, f)
  }

  fn exec_drop<Q, P>(&mut self, query: Q, params: P) -> DeriveSqlResult<()>
  where Q: AsRef<str>,
        P: Into<::mysql::Params>
  {
    self.log(query.as_ref());
    self.conn_mut()?.exec_drop(query, params)
  }
}

