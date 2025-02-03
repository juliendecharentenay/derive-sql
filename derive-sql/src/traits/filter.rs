use super::*;

/// Trait to be implemented for filtering. Returns the contents of a `WHERE` clause.
pub trait FlavoredFilter {
  fn filter<C, R>(&self, conn: &C) -> Result<String>
  where C: Connection<R>,
        R: Row;
}

/// Trait to be implemented for filtering. Returns the contents of a `WHERE` clause.
pub trait Filter {
  fn filter(&self) -> String;
}

impl<F> FlavoredFilter for F
where F: Filter
{
  fn filter<C, R>(&self, _conn: &C) -> Result<String>
  where C: Connection<R>,
        R: Row,
  {
    Ok(Filter::filter(self))
  }
}
