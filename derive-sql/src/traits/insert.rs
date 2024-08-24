use super::*;

pub trait InsertStatement {
  fn insert_stmt(&self) -> Result<String>;
}

pub trait Insert<C, R, T> 
where C: Connection<R>,
      R: Row,
{
  fn insert(&self, conn: &mut C, object: &T) -> Result<()>;
}

pub trait InsertMultiple<'a, C, R, T: 'a>
where C: Connection<R>,
      R: Row,
{
  fn insert_multiple<I>(&self, conn: &mut C, objects: I) -> Result<()>
  where I: core::iter::IntoIterator<Item = &'a T>;
}
