use super::*;

pub struct SimpleOffset {
  limit: Option<usize>,
  offset: Option<usize>,
  next: Option<Box<dyn Selectable>>,
}

impl SimpleOffset {
  pub fn and(mut self, next: Box<dyn Selectable>) -> SimpleOffset { self.next = Some(next); self }
}

impl Selectable for SimpleOffset {
  fn filter(&self) -> Option<String> { self.next.as_ref().and_then(|n| n.filter()) }
  fn limit(&self) -> Option<usize> { self.limit.as_ref().cloned() }
  fn offset(&self) -> Option<usize> { self.offset.as_ref().cloned() }
  fn order_by(&self) -> Option<String> { self.next.as_ref().and_then(|n| n.order_by()) }
}

impl std::convert::TryFrom<()> for SimpleOffset {
  type Error = Box<dyn std::error::Error>;
  fn try_from(_: ()) -> std::result::Result<Self, Self::Error> {
    Ok(SimpleOffset { offset: None, limit: None, next: None })
  }
}

impl std::convert::TryFrom<(usize, usize)> for SimpleOffset {
  type Error = Box<dyn std::error::Error>;
  fn try_from((limit, offset): (usize, usize)) -> std::result::Result<Self, Self::Error> {
    Ok(SimpleOffset { limit: Some(limit), offset: Some(offset), next: None})
  }
}

