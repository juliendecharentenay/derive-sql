use super::*;

/// Define an empty ordering, ie a order clause that is empty and does not trigger any ordering
#[derive(Default)]
pub struct None {}

impl traits::Order for None {
  /// Returns an empty string
  fn as_order_clause(&self) -> String { String::default() }
}
