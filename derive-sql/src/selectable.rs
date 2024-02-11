mod value; // use value::Value;
mod filter;

mod simplefilter; pub use simplefilter::{SimpleFilter};
mod simplelimit; pub use simplelimit::{SimpleLimit};
mod simpleoffset; pub use simpleoffset::{SimpleOffset};
mod simpleorder; pub use simpleorder::{SimpleOrder, Order};

/// Definition of trait `Selectable`. This trait outputs filter, limit and offset statement.
pub trait Selectable {
  fn statement(&self) -> String {
    vec! [
      self.filter().map(|f| format!("WHERE {f}")).unwrap_or_default(),
      self.order_by().map(|v| format!("ORDER BY {v}")).unwrap_or_default(),
      self.limit().map(|v|  format!("LIMIT {v}")).unwrap_or_default(),
      self.offset().map(|v| format!("OFFSET {v}")).unwrap_or_default(),
    ].into_iter().filter(|v| v.len() > 0).collect::<Vec<_>>().join(" ")
  }

  /// Output filter statement - which will be appended to the `WHERE` clause - such as:
  /// * `field='banana'`: item where the entry `field` is equal to `'banana'`;
  /// * `1=0`: return no items;
  /// * `1=1`: return all items;
  ///
  /// Return `None` if no filtering is requested.
  fn filter(&self) -> Option<String>;

  /// Output limit statement - which will be appended to the `LIMIT` clause - such as:
  /// * `10`: returns only 10 entries
  ///
  /// Return `None` if no limit is requested.
  fn limit(&self)  -> Option<usize>;

  /// Output offset statement - which will be appended to the `OFFSET` clause - such as:
  /// * `5`: skip the first 5 entry
  ///
  /// Return `None` if no offset is requested.
  fn offset(&self) -> Option<usize>;

  /// Output order by statement - which will be appended to the `ORDER BY` clause - such as:
  /// * `name DESC`: order by descending name
  ///
  /// Return `None` if no order is requested
  fn order_by(&self) -> Option<String>;
}
