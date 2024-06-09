use super::*;

#[derive(Default)]
pub struct Statement {}

impl derive_sql::traits::AsStatement<()> for Statement {
  fn as_statement(&self) -> derive_sql::DeriveSqlResult<String> {
    Ok(format!("
    CREATE TABLE IF NOT EXISTS test_table
      (
        code TEXT,
        value INT
      )
    ",
    ))
  }
}
