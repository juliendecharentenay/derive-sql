use super::*;

pub struct DatabaseOwned<C, S, I>
where C: crate::sqlite::SqliteTrait,
      S: traits::AsStatement,
      I: for<'b> std::convert::TryFrom<&'b rusqlite::Row<'b>, Error = rusqlite::Error>,
{
  conn: C,
  statement: S,
  phantom_i: std::marker::PhantomData<I>,
}

impl<C, S, I> DatabaseOwned<C, S, I> 
where C: crate::sqlite::SqliteTrait,
      S: traits::AsStatement,
      I: for<'b> std::convert::TryFrom<&'b rusqlite::Row<'b>, Error = rusqlite::Error>,
{
  pub fn from_conn_statement(conn: C, statement: S) -> DatabaseOwned<C, S, I> {
    DatabaseOwned {
      conn,
      statement,
      phantom_i: std::marker::PhantomData,
    }
  }
}

impl<C, S, I> traits::Selectable for DatabaseOwned<C, S, I> 
where C: crate::sqlite::SqliteTrait,
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

