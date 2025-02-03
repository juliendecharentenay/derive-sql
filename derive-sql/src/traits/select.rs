use super::*;

pub trait SelectFlavoredStatement {
  /// SQL statement to retrieve the list of items stored in database
  fn select_stmt<C, R>(&self, conn: &C) -> Result<String>
  where C: Connection<R>, R: Row;

  /// SQL statement to retrieve the list of items stored with specifying a limit and an offset
  fn select_with_filter_stmt<C, R, F>(&self, conn: &C, filter: &F) -> Result<String> 
  where C: Connection<R>, R: Row, F: traits::FlavoredFilter,
  {
    self.select_with_filter_order_limit_offset_options_stmt::<_, _, _, structs::order::None>(conn, Some(filter), None, None, None)
  }

  /// SQL statement to retrieve the list of items stored with specifying a limit and an offset
  fn select_with_filter_order_stmt<C, R, F, O>(&self, conn: &C, filter: &F, order: &O) -> Result<String> 
  where C: Connection<R>, R: Row, F: traits::FlavoredFilter, O: traits::FlavoredOrder,
  {
    self.select_with_filter_order_limit_offset_options_stmt(conn, Some(filter), Some(order), None, None)
  }

  /// SQL statement to retrieve the list of items stored with specifying a limit and an offset
  fn select_with_filter_order_limit_offset_stmt<C, R, F, O>(&self, conn: &C, filter: &F, order: &O, limit: usize, offset: usize) -> Result<String> 
  where C: Connection<R>, R: Row, F: traits::FlavoredFilter, O: traits::FlavoredOrder,
  {
    self.select_with_filter_order_limit_offset_options_stmt(conn, Some(filter), Some(order), Some(limit), Some(offset))
  }

  /// SQL statement to retrieve the list of items stored with specifying optionals filter, limit and offset
  fn select_with_filter_order_limit_offset_options_stmt<C, R, F, O>(&self, conn: &C, filter: Option<&F>, order: Option<&O>, limit: Option<usize>, offset: Option<usize>) -> Result<String> 
  where C: Connection<R>, R: Row, F: traits::FlavoredFilter, O: traits::FlavoredOrder,
  {
    statement_with_conn_filter_order_limit_offset_options(self.select_stmt(conn)?,
      conn, filter, order, limit, offset)
  }
}

pub trait SelectStatement
{
  /// SQL statement to retrieve the list of items stored in database
  fn select_stmt(&self) -> Result<String>;

  /// SQL statement to retrieve the list of items stored with specifying a limit and an offset
  fn select_with_filter_stmt<F>(&self, filter: &F) -> Result<String> 
  where F: traits::Filter,
  {
    self.select_with_filter_order_limit_offset_options_stmt::<_, structs::order::None>(Some(filter), None, None, None)
  }

  /// SQL statement to retrieve the list of items stored with specifying a limit and an offset
  fn select_with_filter_order_stmt<F, O>(&self, filter: &F, order: &O) -> Result<String> 
  where F: traits::Filter, O: traits::Order,
  {
    self.select_with_filter_order_limit_offset_options_stmt(Some(filter), Some(order), None, None)
  }

  /// SQL statement to retrieve the list of items stored with specifying a limit and an offset
  fn select_with_filter_order_limit_offset_stmt<F, O>(&self, filter: &F, order: &O, limit: usize, offset: usize) -> Result<String> 
  where F: traits::Filter, O: traits::Order,
  {
    self.select_with_filter_order_limit_offset_options_stmt(Some(filter), Some(order), Some(limit), Some(offset))
  }

  /// SQL statement to retrieve the list of items stored with specifying optionals filter, limit and offset
  fn select_with_filter_order_limit_offset_options_stmt<F, O>(&self, filter: Option<&F>, order: Option<&O>, limit: Option<usize>, offset: Option<usize>) -> Result<String> 
  where F: traits::Filter, O: traits::Order,
  {
    statement_with_filter_order_limit_offset_options(self.select_stmt()?,
      filter, order, limit, offset)
  }
}

impl<S> SelectFlavoredStatement for S
where S: SelectStatement
{
  fn select_stmt<C, R>(&self, _conn: &C) -> Result<String>
  where C: Connection<R>, R: Row,
  {
    SelectStatement::select_stmt(self)
  }
}

pub trait Select<C, R, T> 
where C: Connection<R>,
      R: Row,
{
  /// Retrieve the list of items of the type `T` stored in database
  fn select(&self, conn: &mut C) -> Result<Vec<T>>;

  /// Retrieve the list of items of the type `T` stored in database matching the filtering criteria
  fn select_with_filter<F>(&self, conn: &mut C, filter: &F) -> Result<Vec<T>>
  where F: traits::FlavoredFilter;

  /// Retrieve the list of items of the type `T` stored in database matching the filtering criteria
  fn select_with_filter_order<F, O>(&self, conn: &mut C, filter: &F, order: &O) -> Result<Vec<T>>
  where F: traits::FlavoredFilter, O: traits::FlavoredOrder;

  /// Retrieve the list of items of the type `T` stored in database
  fn select_with_filter_order_limit_offset<F, O>(&self, conn: &mut C, filter: &F, order: &O, limit: usize, offset: usize) -> Result<Vec<T>>
  where F: traits::FlavoredFilter, O: traits::FlavoredOrder;
}

