use super::*;

pub struct SimpleLimit {
  limit: Option<usize>,
  next: Option<Box<dyn Selectable>>,
}

impl SimpleLimit {
  pub fn and(mut self, next: Box<dyn Selectable>) -> SimpleLimit { self.next = Some(next); self }
}

impl Selectable for SimpleLimit {
  fn filter(&self) -> Option<String> { self.next.as_ref().and_then(|n| n.filter()) }
  fn limit(&self) -> Option<usize> { self.limit.as_ref().cloned() }
  fn offset(&self) -> Option<usize> { self.next.as_ref().and_then(|n| n.offset()) }
  fn order_by(&self) -> Option<String> { self.next.as_ref().and_then(|n| n.order_by()) }
}

impl std::convert::TryFrom<()> for SimpleLimit {
  type Error = Box<dyn std::error::Error>;
  fn try_from(_: ()) -> std::result::Result<Self, Self::Error> {
    Ok(SimpleLimit { limit: None, next: None })
  }
}

impl std::convert::TryFrom<usize> for SimpleLimit {
  type Error = Box<dyn std::error::Error>;
  fn try_from(v: usize) -> std::result::Result<Self, Self::Error> {
    Ok(SimpleLimit { limit: Some(v), next: None})
  }
}

