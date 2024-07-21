// use super::*;

/// Order trait to specify ordering clause
pub trait Order {
  fn as_order_clause(&self) -> String;
}
