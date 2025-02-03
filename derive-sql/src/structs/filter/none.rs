use super::*;

/// Define an empty filter, ie a filter clause that is empty and does not trigger any filtering
#[derive(Default)]
pub struct None {}

impl traits::FlavoredFilter for None {
  /// Returns an empty string
  fn filter<C, R>(&self, _: &C) -> Result<String>
  where C: traits::Connection<R>, R: traits::Row,
  { Ok(String::default()) }
}
