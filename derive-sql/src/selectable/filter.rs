use super::*;

// mod operator; pub use operator::Operator;
pub mod generic; // pub use equal::Equal;
pub use generic::{Operator, Filter};
mod and; pub use and::And;
mod or;  pub use or::Or;

pub trait FilterTrait {
  fn filter(&self) -> String;
}

pub struct Filterable<T> 
where T: FilterTrait
{
  filter: Option<T>,
  next: Option<Box<dyn Selectable>>,
}

impl<T> std::convert::From<()> for Filterable<T>
where T: FilterTrait
{
  fn from(_: ()) -> Self {
    Filterable { filter: None, next: None }
  }
}

impl<T> std::convert::From<T> for Filterable<T> 
where T: FilterTrait
{
  fn from(v: T) -> Self { 
    Filterable { filter: Some(v), next: None }
  }
}

impl<T> Filterable<T> 
where T: FilterTrait
{
  pub fn and(mut self, next: Box<dyn Selectable>) -> Filterable<T> {
    self.next = Some(next);
    self
  }

  pub fn next(&self) -> &Option<Box<dyn Selectable>> { &self.next }
}

/*
pub trait Filterable {
  fn filter(&self) -> Option<String>;
  fn next(&self) -> &Option<Box<dyn Selectable>>;
  fn and(self, next: Box<dyn Selectable>) -> Self;
}
*/

impl<T> Selectable for Filterable<T>
where T: FilterTrait
{
  fn filter(&self)   -> Option<String> { 
    self.filter.as_ref().map(|f| f.filter())
  }

  fn limit(&self)    -> Option<usize>  { self.next.as_ref().and_then(|n| n.limit()) }
  fn offset(&self)   -> Option<usize>  { self.next.as_ref().and_then(|n| n.offset()) }
  fn order_by(&self) -> Option<String> { self.next.as_ref().and_then(|n| n.order_by()) }
}

