//! Implement a generic filter operator to support filter such as `name="John"` as a combination of `(key, operator, value)`
//! Supported operators are `=`, `<`, `<=`, `>`, `>=`
use super::*;
use value::Value;

pub enum Operator {
  Equal,
  Lower,
  LowerEqual,
  Greater,
  GreaterEqual,
}

impl std::fmt::Display for Operator {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Operator::Equal        => write!(f, "="),
      Operator::Lower        => write!(f, "<"),
      Operator::LowerEqual   => write!(f, "<="),
      Operator::Greater      => write!(f, ">"),
      Operator::GreaterEqual => write!(f, ">="),
    }
  }
}

pub struct Filter<T> 
where T: std::fmt::Display
{
  key: String,
  value: Value<T>,
  operator: Operator,
}

impl<T> std::convert::From<(String, Operator, Value<T>)> for Filter<T> 
where T: std::fmt::Display
{
  fn from((key, operator, value): (String, Operator, Value<T>)) -> Self {
    Filter { key, value, operator }
  }
}

impl<T> std::convert::From<(&str, Operator, Value<T>)> for Filter<T> 
where T: std::fmt::Display
{
  fn from((key, operator, value): (&str, Operator, Value<T>)) -> Self {
    Filter { key: key.to_string(), value, operator }
  }
}

impl<T> FilterTrait for Filter<T>
where T: std::fmt::Display
{
  fn filter(&self) -> String {
    format!("`{0}` {1} {2}", self.key, self.operator, self.value)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_outputs_correct_statement() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let g: Filter<u32> = ("user_id", Operator::Equal, 1.into()).into();
    assert!(FilterTrait::filter(&g).eq("`user_id` = 1"));
    Ok(())
  }
}

/*
pub struct Generic<T> 
where T: std::fmt::Display
{
  filter: Option<Filter<T>>,
}

impl<T> std::convert::From<()> for Generic<T>
where T: std::fmt::Display
{
  fn from(_: ()) -> Self {
    Generic { filter: None, next: None }
  }
}

impl<T> std::convert::From<(String, Operator, Value<T>)> for Generic<T> 
where T: std::fmt::Display
{
  fn from((key, operator, value): (String, Operator, Value<T>)) -> Self {
    Generic { filter: Some(Filter { key, value, operator }), next: None }
  }
}

impl<T> std::convert::From<(&str, Operator, Value<T>)> for Generic<T> 
where T: std::fmt::Display
{
  fn from((key, operator, value): (&str, Operator, Value<T>)) -> Self {
    Generic { filter: Some(Filter { key: key.to_string(), value, operator }), next: None }
  }
}

impl<T> Filterable for Generic<T>
where T: std::fmt::Display
{
  fn filter(&self) -> Option<String> {
    self.filter
    .as_ref()
    .map(|v| format!("`{0}` {1} {2}", v.key, v.operator, v.value))
  }

  fn next(&self) -> &Option<Box<dyn Selectable>> { &self.next }
  fn and(mut self, next: Box<dyn Selectable>)  -> Self { self.next = Some(next); self }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_outputs_correct_statement() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let g: Generic<u32> = ("user_id", Operator::Equal, 1.into()).into();
    assert!(filter::Filterable::filter(&g).unwrap().eq("`user_id` = 1"));
    assert!(g.next().is_none());

    Ok(())
  }
}
*/
