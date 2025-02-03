use super::*;

/// Struct to facilitate the definiion of a condition associated with a given field
/// For example, create a condition to filter name column equal to 'Jane':
///
/// ```rust
/// # #[cfg(feature = "sqlite")]
/// # fn wrapper() {
/// let conn = rusqlite::Connection::open_in_memory().unwrap();
/// use derive_sql::structs::{Field, filter, order};
/// use derive_sql::traits::{FlavoredFilter, FlavoredOrder};
///
/// let condition: filter::Condition<_> = Field::from("name").eq("Jane");
/// assert!(condition.filter(&conn).unwrap().eq("`name` = 'Jane'"));
///
/// let condition: order::Condition = Field::from("name").ascending();
/// assert!(condition.as_order_clause(&conn).unwrap().eq("`name` ASC"));
///
/// let condition: filter::Condition<_> = Field::from_table_column("table", "col").eq("val");
/// assert!(condition.filter(&conn).unwrap().eq("`table`.`col` = 'val'"));
/// # }()
/// ```
pub struct Field {
  table: Option<String>,
  label: String,
}

impl Field {
  /// Create a new field with the given label
  pub fn from(label: &str) -> Field {
    Field { table: None, label: label.to_string() }
  }

  /// Create a new field with the given label
  pub fn from_table_column(table: &str, label: &str) -> Field {
    Field { table: Some(table.to_string()), label: label.to_string() }
  }

  /// Generate a test on Null
  pub fn is_none(self) -> filter::Condition<bool> {
    filter::Condition::from_table_label_operator(
      self.table,
      self.label,
      filter::Operator::IsNull,
    )
  }

  /// Generate a test on Not Null
  pub fn is_some(self) -> filter::Condition<bool> {
    filter::Condition::from_table_label_operator(
      self.table,
      self.label,
      filter::Operator::IsNotNull,
    )
  }

  /// Generate an 'equal' condition
  pub fn eq<T>(self, t: T) -> filter::Condition<T> 
  where filter::Value<T>: std::convert::From<T>,
        T: std::fmt::Display,
  {
    filter::Condition::from_table_label_operator(
      self.table,
      self.label,
      filter::Operator::Equal(t.into()),
    )
  }

  /// Generate a 'not-equal' condition
  pub fn ne<T>(self, t: T) -> filter::Condition<T> 
  where filter::Value<T>: std::convert::From<T>,
        T: std::fmt::Display,
  {
    filter::Condition::from_table_label_operator(
      self.table,
      self.label,
      filter::Operator::NotEqual(t.into()),
    )
  }

  /// Generate a 'greater than' condition
  pub fn gt<T>(self, t: T) -> filter::Condition<T>
  where filter::Value<T>: std::convert::From<T>,
        T: std::fmt::Display
  {
    filter::Condition::from_table_label_operator(
      self.table,
      self.label,
      filter::Operator::GreaterThan(t.into()),
    )
  }

  /// Generate a 'greater equal' condition
  pub fn ge<T>(self, t: T) -> filter::Condition<T>
  where filter::Value<T>: std::convert::From<T>,
        T: std::fmt::Display
  {
    filter::Condition::from_table_label_operator(
      self.table,
      self.label,
      filter::Operator::GreaterEqual(t.into()),
    )
  }

  /// Generate a 'lower than' condition
  pub fn lt<T>(self, t: T) -> filter::Condition<T>
  where filter::Value<T>: std::convert::From<T>,
        T: std::fmt::Display
  {
    filter::Condition::from_table_label_operator(
      self.table,
      self.label,
      filter::Operator::LowerThan(t.into()),
    )
  }

  /// Generate a 'lower equal' condition
  pub fn le<T>(self, t: T) -> filter::Condition<T>
  where filter::Value<T>: std::convert::From<T>,
        T: std::fmt::Display
  {
    filter::Condition::from_table_label_operator(
      self.table,
      self.label,
      filter::Operator::LowerEqual(t.into()),
    )
  }

  /// Generate an ascending order clause condition
  pub fn ascending(self) -> order::Condition
  {
    order::Condition::from_table_label_operator(
      self.table,
      self.label,
      order::Operator::Ascending,
    )
  }

  /// Generate a descending order clause condition
  pub fn descending(self) -> order::Condition
  {
    order::Condition::from_table_label_operator(
      self.table,
      self.label,
      order::Operator::Descending,
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  type Row = traits::tests::Row;

  #[test]
  fn it_display_correct_clause_for_null_not_null_tests() -> Result<()> {
    use traits::FlavoredFilter;
    let conn = traits::tests::SQLiteFlavoredConnection {};
    assert!(Field::from("key").is_some().filter::<_, Row>(&conn)?.eq("`key` IS NOT NULL"));
    assert!(Field::from("key").is_none().filter::<_, Row>(&conn)?.eq("`key` IS NULL"));
    Ok(())
  }

  #[test]
  fn it_display_correct_clause_for_u32() -> Result<()> {
    use traits::FlavoredFilter;
    let conn = traits::tests::SQLiteFlavoredConnection {};

    assert!(Field::from("key").eq(1u32).filter::<_, Row>(&conn)?.eq("`key` = 1"));
    assert!(Field::from("key").ne(1u32).filter::<_, Row>(&conn)?.eq("`key` != 1"));
    assert!(Field::from("key").gt(2u32).filter::<_, Row>(&conn)?.eq("`key` > 2"));
    assert!(Field::from("key").ge(2u32).filter::<_, Row>(&conn)?.eq("`key` >= 2"));
    assert!(Field::from("key").lt(2u32).filter::<_, Row>(&conn)?.eq("`key` < 2"));
    assert!(Field::from("key").le(2u32).filter::<_, Row>(&conn)?.eq("`key` <= 2"));

    Ok(())
  }

  #[test]
  fn it_display_correct_clause_for_string() -> Result<()> {
    use traits::FlavoredFilter;
    let conn = traits::tests::SQLiteFlavoredConnection {};

    assert!(Field::from("key_str").eq("val").filter::<_, Row>(&conn)?.eq("`key_str` = 'val'"));
    assert!(Field::from("key_str").ne("val").filter::<_, Row>(&conn)?.eq("`key_str` != 'val'"));
    assert!(Field::from("key_str").gt("val").filter::<_, Row>(&conn)?.eq("`key_str` > 'val'"));
    assert!(Field::from("key_str").ge("val").filter::<_, Row>(&conn)?.eq("`key_str` >= 'val'"));
    assert!(Field::from("key_str").lt("val").filter::<_, Row>(&conn)?.eq("`key_str` < 'val'"));
    assert!(Field::from("key_str").le("val").filter::<_, Row>(&conn)?.eq("`key_str` <= 'val'"));

    Ok(())
  }

  #[test]
  fn it_display_correct_order_clause() -> Result<()> {
    use traits::FlavoredOrder;
    let conn = traits::tests::SQLiteFlavoredConnection {};

    assert!(Field::from("order").ascending().as_order_clause::<_, Row>(&conn)?.eq("`order` ASC"));
    assert!(Field::from("order").descending().as_order_clause::<_, Row>(&conn)?.eq("`order` DESC"));

    Ok(())
  }
}
