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
}

impl std::convert::TryFrom<()> for SimpleLimit {
  type Error = Box<dyn std::error::Error>;
  fn try_from(_: ()) -> Result<Self, Self::Error> {
    Ok(SimpleLimit { limit: None, next: None })
  }
}

impl std::convert::TryFrom<usize> for SimpleLimit {
  type Error = Box<dyn std::error::Error>;
  fn try_from(v: usize) -> Result<Self, Self::Error> {
    Ok(SimpleLimit { limit: Some(v), next: None})
  }
}

