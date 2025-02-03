use super::*;

pub enum Operator<T> 
where T: std::fmt::Display
{
  IsNull,
  IsNotNull,
  Equal(Value<T>),
  NotEqual(Value<T>),
  GreaterThan(Value<T>),
  GreaterEqual(Value<T>),
  LowerThan(Value<T>),
  LowerEqual(Value<T>),
}

/// Describe a single filtering condition using a label and an operator (with the associated operator value if applicable)
pub struct Condition<T>
where T: std::fmt::Display
{
  table: Option<String>,
  label: String,
  operator: Operator<T>,
}

impl<T> Condition<T>
where T: std::fmt::Display
{
  pub fn from_table_label_operator(table: Option<String>, label: String, operator: Operator<T>) -> Condition<T> {
    Condition { table, label, operator }
  }

  pub fn from_label_operator(label: String, operator: Operator<T>) -> Condition<T> {
    Condition { table: None, label, operator }
  }
}

impl<T> traits::FlavoredFilter for Condition<T>
where T: std::fmt::Display
{
  /// Return the `WHERE` clause associated with the condition
  fn filter<C, R>(&self, conn: &C) -> Result<String>
  where C: traits::Connection<R>, R: traits::Row,
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
      Operator::IsNull          => format!("{label} IS NULL"),
      Operator::IsNotNull       => format!("{label} IS NOT NULL"),
      Operator::Equal(v)        => format!("{label} = {v}"),
      Operator::NotEqual(v)     => format!("{label} != {v}"),
      Operator::GreaterThan(v)  => format!("{label} > {v}"),
      Operator::GreaterEqual(v) => format!("{label} >= {v}"),
      Operator::LowerThan(v)    => format!("{label} < {v}"),
      Operator::LowerEqual(v)   => format!("{label} <= {v}"),
    };
    Ok(r)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[cfg(test)]
  fn it_display_correct_clause_for_u32() -> Result<()> {
    use traits::FlavoredFilter;
    let conn = traits::tests::SQLiteFlavoredConnection {};
    type Row = traits::tests::Row;

    assert!(Field::from("key").eq(1u32).filter::<_, Row>(&conn)?.eq("`key` = 1"));
    assert!(Field::from("key").ne(1u32).filter::<_, Row>(&conn)?.eq("`key` != 1"));
    assert!(Field::from("key").gt(2u32).filter::<_, Row>(&conn)?.eq("`key` > 2"));
    assert!(Field::from("key").ge(2u32).filter::<_, Row>(&conn)?.eq("`key` >= 2"));
    assert!(Field::from("key").lt(2u32).filter::<_, Row>(&conn)?.eq("`key` < 2"));
    assert!(Field::from("key").le(2u32).filter::<_, Row>(&conn)?.eq("`key` <= 2"));

    Ok(())
  }

  #[cfg(test)]
  fn it_display_correct_clause_for_string() -> Result<()> {
    use traits::FlavoredFilter;
    let conn = traits::tests::SQLiteFlavoredConnection {};
    type Row = traits::tests::Row;

    assert!(Field::from("key_str").eq("val").filter::<_, Row>(&conn)?.eq("`key_str` = 'val'"));
    assert!(Field::from("key_str").ne("val").filter::<_, Row>(&conn)?.eq("`key_str` != 'val'"));
    assert!(Field::from("key_str").gt("val").filter::<_, Row>(&conn)?.eq("`key_str` > 'val'"));
    assert!(Field::from("key_str").ge("val").filter::<_, Row>(&conn)?.eq("`key_str` >= 'val'"));
    assert!(Field::from("key_str").lt("val").filter::<_, Row>(&conn)?.eq("`key_str` < 'val'"));
    assert!(Field::from("key_str").le("val").filter::<_, Row>(&conn)?.eq("`key_str` <= 'val'"));

    Ok(())
  }
}
