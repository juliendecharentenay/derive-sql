use super::*;

/// Order trait to specify ordering clause
pub trait FlavoredOrder {
  fn as_order_clause<C, R>(&self, conn: &C) -> Result<String>
  where C: Connection<R>,
        R: Row;
}

/// Order trait to specify ordering clause
pub trait Order {
  fn as_order_clause(&self) -> String;
}

impl<O> FlavoredOrder for O
where O: Order
{
  fn as_order_clause<C, R>(&self, _conn: &C) -> Result<String>
  where C: Connection<R>,
        R: Row,
  {
    Ok(Order::as_order_clause(self))
  }
}
