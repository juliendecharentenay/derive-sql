use super::*;

pub trait DeleteFlavoredStatement {
  fn delete_stmt<C, R>(&self, conn: &C) -> Result<String>
  where C: Connection<R>, 
        R: Row;

  fn delete_with_filter_stmt<C, R, F>(&self, conn: &C, filter: &F) -> Result<String>
  where C: Connection<R>,
        R: Row,
        F: traits::FlavoredFilter,
  {
    statement_with_conn_filter_order_limit_offset_options::<_, _, _, structs::order::None>(self.delete_stmt(conn)?,
      conn, Some(filter), None, None, None)
  }

  fn delete_with_filter_order_limit_offset_stmt<C, R, F, O>(&self, conn: &C, filter: &F, order: &O, limit: usize, offset: usize) -> Result<String>
  where C: Connection<R>,
        R: Row,
        F: traits::FlavoredFilter, 
        O: traits::FlavoredOrder,
  {
    statement_with_conn_filter_order_limit_offset_options(self.delete_stmt(conn)?,
      conn, Some(filter), Some(order), Some(limit), Some(offset))
  }
}

pub trait DeleteStatement {
  fn delete_stmt(&self) -> Result<String>;

  fn delete_with_filter_stmt<F>(&self, filter: &F) -> Result<String>
  where F: traits::Filter
  {
    statement_with_filter_order_limit_offset_options::<_, structs::order::None>(self.delete_stmt()?,
      Some(filter), None, None, None)
  }

  fn delete_with_filter_order_limit_offset_stmt<F, O>(&self, filter: &F, order: &O, limit: usize, offset: usize) -> Result<String>
  where F: traits::Filter, O: traits::Order,
  {
    statement_with_filter_order_limit_offset_options(self.delete_stmt()?,
      Some(filter), Some(order), Some(limit), Some(offset))
  }
}

impl<T> DeleteFlavoredStatement for T
where T: DeleteStatement
{
  fn delete_stmt<C, R>(&self, _conn: &C) -> Result<String>
  where C: Connection<R>,
        R: Row,
  {
    DeleteStatement::delete_stmt(self)
  }
}

pub trait Delete<C, R> 
where C: Connection<R>,
      R: Row,
{
  /// Delete all items of type stored in database
  fn delete(&self, conn: &mut C) -> Result<()>;

  /// Delete all items of type matching the filtering criteria stored in database
  fn delete_with_filter<F>(&self, conn: &mut C, filter: &F) -> Result<()>
  where F: traits::FlavoredFilter;

  /// Delete `limit` items after nominated `offset` items of type matching the filtering criteria and ordered in accordance with order
  /// statement
  fn delete_with_filter_order_limit_offset<F, O>(&self, conn: &mut C, filter: &F, order: &O, limit: usize, offset: usize) -> Result<()>
  where F: traits::FlavoredFilter, O: FlavoredOrder;
}
