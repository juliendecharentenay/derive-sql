//! Define value properties for use in filter
use super::*;

pub enum Value<T>
where T: std::fmt::Display
{
  Escaped(T),
  Raw(T),
}

impl<T> std::fmt::Display for Value<T>
where T: std::fmt::Display
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Value::Escaped(v) => write!(f, "'{v}'"),
      Value::Raw(v) => write!(f, "{v}"),
    }
  }
}

impl std::convert::From<String> for Value<String> {
  fn from(v: String) -> Self {
    Value::Escaped(v)
  }
}

impl<'a> std::convert::From<&'a str> for Value<&'a str> {
  fn from(v: &'a str) -> Self {
    Value::Escaped(v)
  }
}

impl std::convert::From<u32> for Value<u32> {
  fn from(v: u32) -> Self {
    Value::Raw(v)
  }
}


