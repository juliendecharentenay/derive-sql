use super::*;

pub struct Conn {
  conn: rusqlite::Connection,
}

impl std::convert::From<rusqlite::Connection> for Conn {
  fn from(conn: rusqlite::Connection) -> Self { Conn { conn } }
}

impl SqliteTrait for Conn {
  fn execute<P>(&self, sql: &str, params: P) -> DeriveSqlResult<usize>
  where P: rusqlite::Params
  {
    Ok(self.conn.execute(sql, params)?)
  }

  fn query_first<T, P, F>(&self, sql: &str, params: P, f: F) -> DeriveSqlResult<T>
  where P: rusqlite::Params,
        F: FnOnce(&rusqlite::Row<'_>) -> rusqlite::Result<T>
  {
    Ok(
    self.conn.prepare(sql)?
    .query_row(params, f)?
    )
  }

  fn query_map<T, P, F>(&self, sql: &str, params: P, f: F) -> DeriveSqlResult<Vec<T>>
  where P: rusqlite::Params,
        F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>
  {
    Ok(
    self.conn.prepare(sql)?
    .query_map(params, f)?
    .collect::<rusqlite::Result<Vec<T>>>()?
    )
  }
}

