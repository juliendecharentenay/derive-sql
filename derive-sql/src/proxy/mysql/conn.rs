use super::*;

pub struct Conn<C>
where C: ::mysql::prelude::Queryable
{
  conn: C,
}

impl<C> std::convert::From<C> for Conn<C>
where C: ::mysql::prelude::Queryable
{
  fn from(conn: C) -> Self { Conn { conn } }
}

impl<C> MysqlTrait for Conn<C> 
where C: ::mysql::prelude::Queryable
{
  fn query_drop<Q>(&mut self, query: Q) -> DeriveSqlResult<()>
  where Q: AsRef<str>
  {
    Ok(self.conn.query_drop(query)?)
  }

  fn query_first<T, Q>(&mut self, query: Q) -> DeriveSqlResult<Option<T>>
  where Q: AsRef<str>,
        T: ::mysql::prelude::FromRow
  {
    Ok(self.conn.query_first(query)?)
  }

  fn query_map<T, F, Q, U>(&mut self, query: Q, f: F) -> DeriveSqlResult<Vec<U>>
  where Q: AsRef<str>,
        T: ::mysql::prelude::FromRow,
        F: FnMut(T) -> U
  {
    Ok(self.conn.query_map(query, f)?)
  }

  fn exec_drop<Q, P>(&mut self, query: Q, params: P) -> DeriveSqlResult<()>
  where Q: AsRef<str>,
        P: Into<::mysql::Params>
  {
    Ok(self.conn.exec_drop(query, params)?)
  }
}

