use super::*;

pub enum Operator
{
  Ascending,
  Descending,
}

/// Describe a single ordering condition using a label and an operator
pub struct Condition
{
  table: Option<String>,
  label: String,
  operator: Operator,
}

impl Condition
{
  /// Create a condition from a table name, column name and an operator
  pub fn from_table_label_operator(table: Option<String>, label: String, operator: Operator) -> Condition {
    Condition { table, label, operator }
  }

  /// Create a condition from a column name and an operator
  pub fn from_label_operator(label: String, operator: Operator) -> Condition {
    Condition { table: None, label, operator }
  }
}

impl traits::FlavoredOrder for Condition
{
  /// Return the `ORDER BY` clause associated with the condition
  fn as_order_clause<C, R>(&self, conn: &C) -> Result<String> 
  where C: traits::Connection<R>, R: traits::Row
  {
    let flavor = conn.flavor();
    let label = if let Some(table) = &self.table {
      format!("{table}.{label}", 
        table=flavor.table(table)?,
        label=flavor.column(self.label.as_str())?)
    } else {
      format!("{label}",
        label=flavor.column(self.label.as_str())?)
    };
    let r = match &self.operator {
      Operator::Ascending   => format!("{label} ASC"),
      Operator::Descending  => format!("{label} DESC"),
    };
    Ok(r)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[cfg(test)]
  fn it_display_correct_clause() -> Result<()> {
    use traits::FlavoredOrder;
    let conn = traits::tests::SQLiteFlavoredConnection {};
    type Row = traits::tests::Row;

    assert!(Condition::from_label_operator("key".to_string(), Operator::Ascending).as_order_clause::<_, Row>(&conn)?.eq("`key` ASC"));
    assert!(Condition::from_label_operator("key".to_string(), Operator::Descending).as_order_clause::<_, Row>(&conn)?.eq("`key` DESC"));

    Ok(())
  }
}
