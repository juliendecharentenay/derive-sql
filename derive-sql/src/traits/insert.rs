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
