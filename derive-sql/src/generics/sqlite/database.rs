use super::*;

pub struct Database<'a, C, S, I>
where C: proxy::sqlite::SqliteTrait,
      S: traits::AsStatement,
      I: for<'b> std::convert::TryFrom<&'b rusqlite::Row<'b>, Error = rusqlite::Error>,
{
  conn: &'a C,
  statement: S,
  phantom_i: std::marker::PhantomData<I>,
}

impl<'a, C, S, I> std::convert::From<&'a C> for Database<'a, C, S, I>
where C: proxy::sqlite::SqliteTrait,
      S: traits::AsStatement + std::default::Default,
      I: for<'b> std::convert::TryFrom<&'b rusqlite::Row<'b>, Error = rusqlite::Error>
{
  fn from(conn: &'a C) -> Self {
    Database { 
      conn, 
      statement: S::default(),
      phantom_i: std::marker::PhantomData,
    }
  }
}

impl<'a, C, S, I> Database<'a, C, S, I> 
where C: proxy::sqlite::SqliteTrait,
      S: traits::AsStatement,
      I: for<'b> std::convert::TryFrom<&'b rusqlite::Row<'b>, Error = rusqlite::Error>,
{
  pub fn from_conn_statement(conn: &'a C, statement: S) -> Database<'a, C, S, I> {
    Database {
      conn,
      statement,
      phantom_i: std::marker::PhantomData,
    }
  }
}

impl<'a, C, S, I> traits::Selectable for Database<'a, C, S, I> 
where C: proxy::sqlite::SqliteTrait,
      S: traits::AsStatement,
      I: for<'b> std::convert::TryFrom<&'b rusqlite::Row<'b>, Error = rusqlite::Error>,
{
  type Item = I;
  fn select_with_statement(&self, statement: String) -> DeriveSqlResult<Vec<Self::Item>> {
    self.conn.query_map(statement.as_str(), 
      [],
      |r| r.try_into())
  }

  fn statement(&self) -> DeriveSqlResult<String> {
    Ok(self.statement.into_statement()?)
  }
}

