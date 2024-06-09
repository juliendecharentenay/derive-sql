use super::*;

// mod select; pub use select::Select;
// mod connection; pub use connection::Connection;

pub trait AsStatement<I> {
  fn as_statement(&self) -> DeriveSqlResult<String>;
}

pub trait IsSelect {}

pub trait Select<T, I>
where T: AsStatement<I> + IsSelect
{
  fn select(&self, as_statement: &T) -> DeriveSqlResult<Vec<I>>;
}

pub trait CreateTable<T>
where T: AsStatement<()>
{
  fn create_table(&mut self, as_statement: &T) -> DeriveSqlResult<()>;
}

