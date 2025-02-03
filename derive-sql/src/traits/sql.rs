use super::*;

impl<C, R, T, S> update::Update<C, R, T> for S
where S: update::UpdateFlavoredStatement,
      C: Connection<R>,
      R: Row,
      T: params::Params + row::TryFromRefRow<R>,
{
  fn update(&self, conn: &mut C, object: &T) -> Result<()> {
    conn.execute_with_params(self.update_stmt(conn)?, object)?;
    Ok(())
  }

  fn update_with_filter<F>(&self, conn: &mut C, filter: &F, object: &T) -> Result<()>
  where F: traits::FlavoredFilter
  {
    conn.execute_with_params(self.update_with_filter_stmt(conn, filter)?, object)?;
    Ok(())
  }

  fn update_with_filter_order_limit_offset<F, O>(&self, conn: &mut C, filter: &F, order: &O, limit: usize, offset: usize, object: &T) -> Result<()>
  where F: traits::FlavoredFilter, O: FlavoredOrder,
  {
    match conn.flavor() {
      Flavor::SQLite
      | Flavor::MySQL => {
        log::warn!("Update with limit and offset will be deprecated");
        conn.execute_with_params(self.update_with_filter_order_limit_offset_stmt(conn, filter, order, limit, offset)?, object)?;
        Ok(())
      }
      Flavor::PostgreSQL => Err(Error::UpdateWithLimitOffsetNotSupported),
    }
  }
}

impl<C, R, S> delete::Delete<C, R> for S
where S: delete::DeleteFlavoredStatement,
      C: Connection<R>,
      R: Row,
{
  fn delete(&self, conn: &mut C) -> Result<()> {
    conn.query_drop(self.delete_stmt(conn)?)?;
    Ok(())
  }

  fn delete_with_filter<F>(&self, conn: &mut C, filter: &F) -> Result<()> 
  where F: traits::FlavoredFilter
  {
    conn.query_drop(self.delete_with_filter_stmt(conn, filter)?)?;
    Ok(())
  }

  fn delete_with_filter_order_limit_offset<F, O>(&self, conn: &mut C, filter: &F, order: &O, limit: usize, offset: usize) -> Result<()> 
  where F: traits::FlavoredFilter, O: FlavoredOrder,
  {
    conn.query_drop(self.delete_with_filter_order_limit_offset_stmt(conn, filter, order, limit, offset)?)?;
    Ok(())
  }
}

impl<C, R, T, S> insert::Insert<C, R, T> for S
where S: insert::InsertFlavoredStatement,
      T: params::Params,
      C: Connection<R>,
      R: Row,
{
  fn insert(&self, conn: &mut C, object: &T) -> Result<()> {
    conn.execute_with_params(self.insert_stmt(conn)?, object)?;
    Ok(())
  }
}

impl<'a, C, R, T, S> insert::InsertMultiple<'a, C, R, T> for S
where S: insert::InsertFlavoredStatement,
      T: params::Params + 'a,
      C: Connection<R>,
      R: Row,
{
  fn insert_multiple<I>(&self, conn: &mut C, objects: I) -> Result<()> 
  where I: core::iter::IntoIterator<Item = &'a T>
  {
    conn.execute_with_params_iterator(self.insert_stmt(conn)?, objects)?;
    Ok(())
  }
}

impl<C, R, T, S> select::Select<C, R, T> for S
where S: select::SelectFlavoredStatement,
      T: row::TryFromRefRow<R>,
      C: Connection<R>,
      R: Row,
{
  fn select(&self, conn: &mut C) -> Result<Vec<T>> {
    conn.query_try_as_object(self.select_stmt(conn)?)
  }

  fn select_with_filter<F>(&self, conn: &mut C, filter: &F) -> Result<Vec<T>>
  where F: traits::FlavoredFilter
  {
    conn.query_try_as_object(self.select_with_filter_stmt(conn, filter)?)
  }

  fn select_with_filter_order<F, O>(&self, conn: &mut C, filter: &F, order: &O) -> Result<Vec<T>>
  where F: traits::FlavoredFilter, O: traits::FlavoredOrder,
  {
    conn.query_try_as_object(self.select_with_filter_order_stmt(conn, filter, order)?)
  }

  fn select_with_filter_order_limit_offset<F, O>(&self, conn: &mut C, filter: &F, order: &O, limit: usize, offset: usize) -> Result<Vec<T>> 
  where F: traits::FlavoredFilter, O: traits::FlavoredOrder,
  {
    conn.query_try_as_object(self.select_with_filter_order_limit_offset_stmt(conn, filter, order, limit, offset)?)
  }
}

impl<C, R, S> table::Table<C, R> for S
where S: table::TableFlavoredStatement,
      C: Connection<R>,
      R: Row,
{
  fn create(&self, conn: &mut C) -> Result<()> {
    conn.query_drop(self.create_stmt(conn)?)
  }

  fn create_if_not_exist(&self, conn: &mut C) -> Result<()> {
    conn.query_drop(self.create_if_not_exist_stmt(conn)?)
  }

  fn drop(&self, conn: &mut C) -> Result<()> {
    conn.query_drop(self.drop_stmt(conn)?)
  }
}
