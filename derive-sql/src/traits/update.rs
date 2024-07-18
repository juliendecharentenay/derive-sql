use super::*;

pub trait UpdateStatement {
  fn update_stmt(&self) -> Result<String>;

  fn update_with_filter_stmt<F>(&self, filter: &F) -> Result<String>
  where F: traits::Filter
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

pub trait Update<C, R, T> 
where C: Connection<R>,
      R: Row,
{
  fn update(&self, conn: &mut C, object: &T) -> Result<()>;

  fn update_with_filter<F>(&self, conn: &mut C, filter: &F, object: &T) -> Result<()>
  where F: traits::Filter;

  fn update_with_filter_order_limit_offset<F, O>(&self, conn: &mut C, filter: &F, order: &O, limit: usize, offset: usize, object: &T) -> Result<()>
  where F: traits::Filter, O: Order;

}
