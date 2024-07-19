use super::*;

#[derive(Debug, serde::Serialize)]
pub struct Item {
  nationality: String,
  count: u32,
}

impl std::convert::TryFrom<&rusqlite::Row<'_>> for Item {
  type Error = rusqlite::Error;
  fn try_from(row: &rusqlite::Row<'_>) -> std::result::Result<Self, rusqlite::Error> {
    Ok( Item {
      nationality: row.get(0)?,
      count: row.get(1)?,
    })
  }
}

#[derive(Default)]
pub struct Statement {}

impl derive_sql::traits::IsSelect for Statement {}

impl derive_sql::traits::AsStatement<Item> for Statement {
  fn as_statement(&self) -> derive_sql::DeriveSqlResult<String> {
    Ok(format!("
      SELECT {nationality}, COUNT({nationality}) AS count FROM {table} 
      WHERE {nationality} IS NOT NULL
      GROUP BY {nationality} 
      ORDER BY count DESC
    ", 
    nationality = artist::ArtistSqlite::<derive_sql::proxy::sqlite::Conn>::NATIONALITY,
    table       = artist::ArtistSqlite::<derive_sql::proxy::sqlite::Conn>::TABLE_NAME,
    ))
  }
}

