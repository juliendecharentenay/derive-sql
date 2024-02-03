use super::*;

/// Nominate whether the order is ascending (ie A to Z) or descending (ie Z to A)
pub enum Order {
  Ascending,
  Descending,
}

pub struct SimpleOrder {
  order: Option<(String, Order)>,
  next: Option<Box<dyn Selectable>>,
}

impl SimpleOrder {
  pub fn and(mut self, next: Box<dyn Selectable>) -> SimpleOrder { self.next = Some(next); self }
}

impl Selectable for SimpleOrder {
  fn filter(&self) -> Option<String> { self.next.as_ref().and_then(|n| n.filter()) }
  fn limit(&self) -> Option<usize>   { self.next.as_ref().and_then(|n| n.limit()) }
  fn offset(&self) -> Option<usize>  { self.next.as_ref().and_then(|n| n.offset()) }
  fn order_by(&self) -> Option<String> {
    self.order.as_ref()
    .map(|(k, o)|
      match o {
        Order::Ascending  => format!("{k} ASC"),
        Order::Descending => format!("{k} DESC"),
      }
    )
  }
}

impl std::convert::TryFrom<()> for SimpleOrder {
  type Error = Box<dyn std::error::Error>;
  fn try_from(_: ()) -> Result<Self, Self::Error> {
    Ok(SimpleOrder { order: None, next: None })
  }
}

impl std::convert::TryFrom<(&str, Order)> for SimpleOrder {
  type Error = Box<dyn std::error::Error>;
  fn try_from((key, order): (&str, Order)) -> Result<Self, Self::Error> {
    Ok(SimpleOrder { order: Some((key.to_string(), order)), next: None })
  }
}

impl std::convert::TryFrom<(String, Order)> for SimpleOrder {
  type Error = Box<dyn std::error::Error>;
  fn try_from((key, order): (String, Order)) -> Result<Self, Self::Error> {
    Ok(SimpleOrder { order: Some((key, order)), next: None })
  }
}
