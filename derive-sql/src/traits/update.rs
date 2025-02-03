use super::*;

pub trait UpdateFlavoredStatement {
  fn update_stmt<C, R>(&self, conn: &C) -> Result<String>
  where C: Connection<R>, R: Row;

  fn update_with_filter_stmt<C, R, F>(&self, conn: &C, filter: &F) -> Result<String>
  where C: Connection<R>, R: Row, F: traits::FlavoredFilter
  {
    statement_with_conn_filter_order_limit_offset_options::<_, _, _, structs::order::None>(self.update_stmt(conn)?,
      conn, Some(filter), None, None, None)
  }

  fn update_with_filter_order_limit_offset_stmt<C, R, F, O>(&self, conn: &C, filter: &F, order: &O, limit: usize, offset: usize) -> Result<String>
  where C: Connection<R>, R: Row, F: FlavoredFilter, O: FlavoredOrder,
  {
    statement_with_conn_filter_order_limit_offset_options(self.update_stmt(conn)?,
      conn, Some(filter), Some(order), Some(limit), Some(offset))
  }
}

pub trait UpdateStatement {
  fn update_stmt(&self) -> Result<String>;

  fn update_with_filter_stmt<F>(&self, filter: &F) -> Result<String>
  where F: Filter
  {
    statement_with_filter_order_limit_offset_options::<_, structs::order::None>(self.update_stmt()?,
    Some(filter), None, None, None)
  }

  fn update_with_filter_order_limit_offset_stmt<F, O>(&self, filter: &F, order: &O, limit: usize, offset: usize) -> Result<String>
  where F: Filter, O: Order,
  {
    statement_with_filter_order_limit_offset_options(self.update_stmt()?,
    Some(filter), Some(order), Some(limit), Some(offset))
  }
}

impl<U> UpdateFlavoredStatement for U
where U: UpdateStatement
{
  fn update_stmt<C, R>(&self, _conn: &C) -> Result<String>
  where C: Connection<R>, R: Row
  {
    UpdateStatement::update_stmt(self)
  }
}

pub trait Update<C, R, T> 
where C: Connection<R>,
      R: Row,
{
  fn update(&self, conn: &mut C, object: &T) -> Result<()>;

  fn update_with_filter<F>(&self, conn: &mut C, filter: &F, object: &T) -> Result<()>
  where F: traits::FlavoredFilter;

  fn update_with_filter_order_limit_offset<F, O>(&self, conn: &mut C, filter: &F, order: &O, limit: usize, offset: usize, object: &T) -> Result<()>
  where F: traits::FlavoredFilter, O: FlavoredOrder;

}
