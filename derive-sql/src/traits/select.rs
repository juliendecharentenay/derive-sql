use super::*;

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

pub trait Select<C, R, T> 
where C: Connection<R>,
      R: Row,
{
  /// Retrieve the list of items of the type `T` stored in database
  fn select(&self, conn: &mut C) -> Result<Vec<T>>;

  /// Retrieve the list of items of the type `T` stored in database matching the filtering criteria
  fn select_with_filter<F>(&self, conn: &mut C, filter: &F) -> Result<Vec<T>>
  where F: traits::Filter;

  /// Retrieve the list of items of the type `T` stored in database matching the filtering criteria
  fn select_with_filter_order<F, O>(&self, conn: &mut C, filter: &F, order: &O) -> Result<Vec<T>>
  where F: traits::Filter, O: traits::Order;

  /// Retrieve the list of items of the type `T` stored in database
  fn select_with_filter_order_limit_offset<F, O>(&self, conn: &mut C, filter: &F, order: &O, limit: usize, offset: usize) -> Result<Vec<T>>
  where F: traits::Filter, O: traits::Order;
}

