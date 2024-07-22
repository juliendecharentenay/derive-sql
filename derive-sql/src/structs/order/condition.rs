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

impl traits::Order for Condition
{
  /// Return the `ORDER BY` clause associated with the condition
  fn as_order_clause(&self) -> String {
    let label = if let Some(table) = &self.table {
      format!("`{table}`.`{label}`", label=&self.label)
    } else {
      format!("`{label}`", label=&self.label)
    };
    match &self.operator {
      Operator::Ascending   => format!("{label} ASC"),
      Operator::Descending  => format!("{label} DESC"),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_display_correct_clause() -> Result<()> {
    use traits::Order;

    assert!(Condition::from_label_operator("key".to_string(), Operator::Ascending).as_order_clause().eq("`key` ASC"));
    assert!(Condition::from_label_operator("key".to_string(), Operator::Descending).as_order_clause().eq("`key` DESC"));

    Ok(())
  }
}
