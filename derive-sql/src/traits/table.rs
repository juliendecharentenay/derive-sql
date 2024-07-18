use super::*;

pub trait TableStatement {
  fn create_stmt(&self) -> Result<String>;
  fn create_if_not_exist_stmt(&self) -> Result<String>;
  fn drop_stmt(&self) -> Result<String>;
}

/// Trait to manage table - create, check if exists, and drop
pub trait Table<C, R>
where C: Connection<R>,
      R: Row,
{
  /// Create a table. Error if the table already exists
  fn create(&self, conn: &mut C) -> Result<()>;

  /// Create a table if it does not already exist
  fn create_if_not_exist(&self, conn: &mut C) -> Result<()>;

  /// Delete the table
  fn drop(&self, conn: &mut C) -> Result<()>;
}
