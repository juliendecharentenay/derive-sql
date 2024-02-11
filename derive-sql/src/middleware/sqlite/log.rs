use super::*;

pub struct Log<C>
where C: SqliteTrait
{
  conn: Option<C>,
}

impl<C> std::default::Default for Log<C>
where C: SqliteTrait
{
  fn default() -> Self { Log { conn: None } }
}

impl<C> Log<C>
where C: SqliteTrait
{
  pub fn with(mut self, conn: C) -> Self { self.conn = Some(conn); self }

  fn log(&self, sql: &str) { ::log::info!("{sql}"); }
}

impl<C> SqliteTrait for Log<C>
where C: SqliteTrait
{
  fn execute<P>(&self, sql: &str, params: P) -> rusqlite::Result<usize>
  where P: rusqlite::Params 
  {
    self.log(sql);
    self.conn.as_ref().ok_or(rusqlite::Error::ModuleError("No SQLite connection provided".to_string()))?
    .execute(sql, params)
  }

  fn query_first<T, P, F>(&self, sql: &str, params: P, f: F) -> rusqlite::Result<T>
  where P: rusqlite::Params,
        F: FnOnce(&rusqlite::Row<'_>) -> rusqlite::Result<T>
  {
    self.log(sql);
    self.conn.as_ref().ok_or(rusqlite::Error::ModuleError("No SQLite connection provided".to_string()))?
    .query_first(sql, params, f)
  }

  fn query_map<T, P, F>(&self, sql: &str, params: P, f: F) -> rusqlite::Result<Vec<T>>
  where P: rusqlite::Params,
        F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>
  {
    self.log(sql);
    self.conn.as_ref().ok_or(rusqlite::Error::ModuleError("No SQLite connection provided".to_string()))?
    .query_map(sql, params, f)
  }
}
  

