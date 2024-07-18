use super::*;

impl<C, R, T, S> update::Update<C, R, T> for S
where S: update::UpdateStatement,
      C: Connection<R>,
      R: Row,
      T: params::Params + row::TryFromRefRow<R>,
{
  fn update(&self, conn: &mut C, object: &T) -> Result<()> {
    conn.execute_with_params(self.update_stmt()?, object)?;
    Ok(())
  }

  fn update_with_filter<F>(&self, conn: &mut C, filter: &F, object: &T) -> Result<()>
  where F: traits::Filter
  {
    conn.execute_with_params(self.update_with_filter_stmt(filter)?, object)?;
    Ok(())
  }

  fn update_with_filter_order_limit_offset<F, O>(&self, conn: &mut C, filter: &F, order: &O, limit: usize, offset: usize, object: &T) -> Result<()>
  where F: traits::Filter, O: Order,
  {
    conn.execute_with_params(self.update_with_filter_order_limit_offset_stmt(filter, order, limit, offset)?, object)?;
    Ok(())
  }
}

impl<C, R, S> delete::Delete<C, R> for S
where S: delete::DeleteStatement,
      C: Connection<R>,
      R: Row,
{
  fn delete(&self, conn: &mut C) -> Result<()> {
    conn.query_drop(self.delete_stmt()?)?;
    Ok(())
  }

  fn delete_with_filter<F>(&self, conn: &mut C, filter: &F) -> Result<()> 
  where F: traits::Filter
  {
    conn.query_drop(self.delete_with_filter_stmt(filter)?)?;
    Ok(())
  }

  fn delete_with_filter_order_limit_offset<F, O>(&self, conn: &mut C, filter: &F, order: &O, limit: usize, offset: usize) -> Result<()> 
  where F: traits::Filter, O: Order,
  {
    conn.query_drop(self.delete_with_filter_order_limit_offset_stmt(filter, order, limit, offset)?)?;
    Ok(())
  }
}

impl<C, R, T, S> insert::Insert<C, R, T> for S
where S: insert::InsertStatement,
      T: params::Params,
      C: Connection<R>,
      R: Row,
{
  fn insert(&self, conn: &mut C, object: &T) -> Result<()> {
    conn.execute_with_params(self.insert_stmt()?, object)?;
    Ok(())
  }
}

impl<C, R, T, S> select::Select<C, R, T> for S
where S: select::SelectStatement,
      T: row::TryFromRefRow<R>,
      C: Connection<R>,
      R: Row,
{
  fn select(&self, conn: &mut C) -> Result<Vec<T>> {
    conn.query_try_as_object(self.select_stmt()?)
  }

  fn select_with_filter<F>(&self, conn: &mut C, filter: &F) -> Result<Vec<T>>
  where F: traits::Filter
  {
    conn.query_try_as_object(self.select_with_filter_stmt(filter)?)
  }

  fn select_with_filter_order<F, O>(&self, conn: &mut C, filter: &F, order: &O) -> Result<Vec<T>>
  where F: traits::Filter, O: traits::Order,
  {
    conn.query_try_as_object(self.select_with_filter_order_stmt(filter, order)?)
  }

  fn select_with_filter_order_limit_offset<F, O>(&self, conn: &mut C, filter: &F, order: &O, limit: usize, offset: usize) -> Result<Vec<T>> 
  where F: traits::Filter, O: traits::Order,
  {
    conn.query_try_as_object(self.select_with_filter_order_limit_offset_stmt(filter, order, limit, offset)?)
  }
}

impl<C, R, S> table::Table<C, R> for S
where S: table::TableStatement,
      C: Connection<R>,
      R: Row,
{
  fn create(&self, conn: &mut C) -> Result<()> {
    conn.query_drop(self.create_stmt()?)
  }

  fn create_if_not_exist(&self, conn: &mut C) -> Result<()> {
    conn.query_drop(self.create_if_not_exist_stmt()?)
  }

  fn drop(&self, conn: &mut C) -> Result<()> {
    conn.query_drop(self.drop_stmt()?)
  }
}
