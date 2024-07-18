use super::*;

/// Define an empty filter, ie a filter clause that is empty and does not trigger any filtering
#[derive(Default)]
pub struct None {}

impl traits::Filter for None {
  /// Returns an empty string
  fn filter(&self) -> String { String::default() }
}
