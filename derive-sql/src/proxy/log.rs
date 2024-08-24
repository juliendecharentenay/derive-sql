use super::*;

/// Implementation of a wrapper that output the statement to as an `info` level log before running the query
/// using the providing connection.
pub struct Log<'a, T, R> 
where T: traits::Connection<R>,
      R: traits::Row,
{
  conn: &'a mut T,
  level: ::log::Level,
  phantom_r: std::marker::PhantomData<R>,
}

impl<'a, T, R> Log<'a, T, R>
where T: traits::Connection<R>,
      R: traits::Row,
{
  pub fn from_connection_level(conn: &'a mut T, level: ::log::Level) -> Log<T, R> {
    Log { conn, level, phantom_r: std::marker::PhantomData, }
  }

  pub fn inner_mut(&mut self) -> &mut T { &mut self.conn }

  pub fn with_level(mut self, level: ::log::Level) -> Log<'a, T, R> {
    self.level = level;
    self
  }

  fn log(&self, statement: &str) {
    match self.level {
      ::log::Level::Error => ::log::error!("{statement}"),
      ::log::Level::Warn  => ::log::warn!("{statement}"),
      ::log::Level::Info  => ::log::info!("{statement}"),
      ::log::Level::Debug => ::log::debug!("{statement}"),
      ::log::Level::Trace => ::log::trace!("{statement}"),
    }
  }
}

impl<'a, T, R> std::convert::From<&'a mut T> for Log<'a, T, R> 
where T: traits::Connection<R>,
      R: traits::Row,
{
  fn from(conn: &'a mut T) -> Self { Log { conn, level: ::log::Level::Info, phantom_r: std::marker::PhantomData, } }
}
  
/*
impl<T, R> std::convert::Into<T> for Log<T, R>
where T: traits::Connection<R>,
      R: traits::Row,
{
  fn into(self) -> T { self.conn }
}

impl<T, R> std::convert::From<Log<T, R>> for T
where T: traits::Connection<R>,
      R: traits::Row,
{
  fn from(v: Log<T, R>) -> Self { v.conn }
}
*/

impl<'a, T, R> traits::Connection<R> for Log<'a, T, R>
where T: traits::Connection<R>,
      R: traits::Row,
{
  fn flavor(&self) -> traits::Flavor { self.conn.flavor() }

  fn execute_with_params<S, P>(&mut self, query: S, params: &P) -> Result<()>
  where S: std::convert::AsRef<str>,
        P: traits::Params,
  {
    self.log(query.as_ref());
    self.conn.execute_with_params(query, params)
  }

  fn execute_with_params_iterator<'b, S, I, P>(&mut self, query: S, params_iter: I) -> Result<()>
  where S: std::convert::AsRef<str>,
        P: traits::Params + 'b,
        I: core::iter::IntoIterator<Item = &'b P>
  {
    self.log(query.as_ref());
    self.conn.execute_with_params_iterator(query, params_iter)
  }

/*
  fn execute_with_params_rows<S, P>(&mut self, query: S, params: &P) -> Result<Vec<R>>
  where S: std::convert::AsRef<str>,
        P: traits::Params,
  {
    self.log(query.as_ref());
    self.conn.execute_with_params_rows(query, params)
  }
  */

  fn query<S>(&mut self, query: S) -> Result<Vec<R>>
  where S: std::convert::AsRef<str>,
  {
    self.log(query.as_ref());
    self.conn.query(query)
  }
}

