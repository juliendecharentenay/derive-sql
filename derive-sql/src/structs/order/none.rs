use super::*;

/// Define an empty ordering, ie a order clause that is empty and does not trigger any ordering
#[derive(Default)]
pub struct None {}

/*
impl traits::FlavoredOrder for None {
  /// Returns an empty string
  fn as_order_clause<C, R>(&self, _: &C) -> Result<String>
  where C: traits::Connection<R>, R: traits::Row,
  { Ok(String::default()) }
}
*/

impl traits::Order for None {
  /// Returns an empty string
  fn as_order_clause(&self) -> String { String::default() }
}
