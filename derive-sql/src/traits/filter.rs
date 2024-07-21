// use super::*;

/// Trait to be implemented for filtering. Returns the contents of a `WHERE` clause.
pub trait Filter {
  fn filter(&self) -> String;
}
