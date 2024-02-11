use super::*;

// mod operator; pub use operator::Operator;
pub mod generic; // pub use equal::Equal;
pub use generic::Operator;

pub trait Filterable {
  fn filter(&self) -> Option<String>;
  fn next(&self) -> &Option<Box<dyn Selectable>>;
  fn and(self, next: Box<dyn Selectable>) -> Self;
}

impl<T> Selectable for T
where T: Filterable
{
  fn filter(&self)   -> Option<String> { Filterable::filter(self) }
  fn limit(&self)    -> Option<usize>  { self.next().as_ref().and_then(|n| n.limit()) }
  fn offset(&self)   -> Option<usize>  { self.next().as_ref().and_then(|n| n.offset()) }
  fn order_by(&self) -> Option<String> { self.next().as_ref().and_then(|n| n.order_by()) }
}

