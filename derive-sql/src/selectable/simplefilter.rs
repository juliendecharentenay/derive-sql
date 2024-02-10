use super::*;

enum Value {
  ValueStr(String),
  ValueU32(u32),
}

pub struct SimpleFilter {
  filter: Option<(String, Value)>,
  next: Option<Box<dyn Selectable>>,
}

impl SimpleFilter {
  pub fn and(mut self, next: Box<dyn Selectable>) -> SimpleFilter { self.next = Some(next); self }
}

impl Selectable for SimpleFilter {
  fn filter(&self) -> Option<String> {
    self.filter.as_ref()
    .map(|(k, v)| 
      match v {
        Value::ValueStr(v) => format!("`{k}` = '{v}'"),
        Value::ValueU32(v) => format!("`{k}` = {v}"),
      }
    )
  }
  fn limit(&self) -> Option<usize> { self.next.as_ref().and_then(|n| n.limit()) }
  fn offset(&self) -> Option<usize> { self.next.as_ref().and_then(|n| n.offset()) }
  fn order_by(&self) -> Option<String> { self.next.as_ref().and_then(|n| n.order_by()) }
}

impl std::convert::TryFrom<()> for SimpleFilter {
  type Error = Box<dyn std::error::Error>;
  fn try_from(_: ()) -> Result<Self, Self::Error> {
    Ok(SimpleFilter { filter: None, next: None })
  }
}

impl std::convert::TryFrom<(String, String)> for SimpleFilter {
  type Error = Box<dyn std::error::Error>;
  fn try_from((key, value): (String, String)) -> Result<Self, Self::Error> {
    Ok(SimpleFilter { filter: Some((key, Value::ValueStr(value))), next: None})
  }
}

impl std::convert::TryFrom<(&str, &str)> for SimpleFilter {
  type Error = Box<dyn std::error::Error>;
  fn try_from((key, value): (&str, &str)) -> Result<Self, Self::Error> {
    Ok(SimpleFilter { filter: Some((key.to_string(), Value::ValueStr(value.to_string()))), next: None, })
  }
}

impl std::convert::TryFrom<(&str, u32)> for SimpleFilter {
  type Error = Box<dyn std::error::Error>;
  fn try_from((key, value): (&str, u32)) -> Result<Self, Self::Error> {
    Ok(SimpleFilter { filter: Some((key.to_string(), Value::ValueU32(value))), next: None, })
  }
}
