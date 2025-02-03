use super::*;

/// Specify table handling statement using the SQL Flavor
pub trait TableFlavoredStatement {
  /// Create table statement
  fn create_stmt<C, R>(&self, conn: &C) -> Result<String> where C: Connection<R>, R: Row;

  /// Create table statement if it does not exists
  fn create_if_not_exist_stmt<C, R>(&self, conn: &C) -> Result<String> where C: Connection<R>, R: Row;

  /// Drop table statemetn
  fn drop_stmt<C, R>(&self, conn: &C) -> Result<String> where C: Connection<R>, R: Row;
}

/// [Deprecated] Specify table handling SQL statement
pub trait TableStatement {
  fn create_stmt(&self) -> Result<String>;
  fn create_if_not_exist_stmt(&self) -> Result<String>;
  fn drop_stmt(&self) -> Result<String>;
}

/// [Deprecated] Blanket statement converting legacy implementation
impl<S> TableFlavoredStatement for S
where S: TableStatement
{
  fn create_stmt<C, R>(&self, _: &C) -> Result<String> where C: Connection<R>, R: Row {
    TableStatement::create_stmt(self)
  }
  fn create_if_not_exist_stmt<C, R>(&self, _: &C) -> Result<String> where C: Connection<R>, R: Row {
    TableStatement::create_if_not_exist_stmt(self)
  }
  fn drop_stmt<C, R>(&self, _: &C) -> Result<String> where C: Connection<R>, R: Row {
    TableStatement::drop_stmt(self)
  }
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
