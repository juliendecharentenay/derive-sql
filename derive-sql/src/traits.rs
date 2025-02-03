//! traits underlying the implementation of SQL functionalities. What some other crates call `prelude`
use super::*;

mod params; pub use params::{Params, Param, ToParam};
mod row; pub use row::{Value, Row, TryFromRefRow, }; // Row2, RefTryInto, };
mod flavor; pub use flavor::{Flavor};

/// Generic trait to be implemented by SQL drivers (or proxy to SQL drivers). This trait is used
/// to provide the basis of the functionalities on which the crate rely
pub trait Connection<R>
where R: Row,
{
  /// Returns flavor of SQL
  fn flavor(&self) -> Flavor;

  /// Implements an `execute` statement
  fn execute_with_params<S, P>(&mut self, query: S, params: &P) -> Result<()>
  where S: std::convert::AsRef<str>,
        P: Params;

  /// Implements an `execute` statement over an iterator of parameters
  fn execute_with_params_iterator<'a, S, I, P>(&mut self, query: S, params_iter: I) -> Result<()>
  where S: std::convert::AsRef<str>,
        P: Params + 'a,
        I: core::iter::IntoIterator<Item = &'a P>;

  /*
  /// Implements an `execute` statement returning the modified elements
  fn execute_with_params_rows<S, P>(&mut self, query: S, params: &P) -> Result<Vec<R>>
  where S: std::convert::AsRef<str>,
        P: Params;
        */

  /// Implements a `query` statement returning a list of results stored as `Row`
  fn query<S>(&mut self, query: S) -> Result<Vec<R>>
  where S: std::convert::AsRef<str>;

  // Implemented methods:
  /*
  /// `execute` statement retuning the list of results as object of the given type `T`
  fn execute_with_params_try_as_object<S, P, T>(&mut self, query: S, params: &P) -> Result<Vec<T>>
  where S: std::convert::AsRef<str>,
        P: Params,
        T: TryFromRefRow<R>,
  {
    Ok(
    self.execute_with_params_rows(query, params)?
    .iter()
    .map(|r: &R| Ok(T::try_from(r)?))
    .collect::<Result<Vec<T>>>()?
    )
  }
  */

/*
  /// `execute` statement retuning the list of results as object of the given type `T`
  fn execute_with_params_as_object<S, P, T>(&mut self, query: S, params: P) -> Result<Vec<T>>
  where S: std::convert::AsRef<str>,
        P: Params,
        T: for<'a> std::convert::From<&'a R>,
  {
    Ok(
    self.execute_with_params(query, params)?
    .iter()
    .map(|r: &R| r.into())
    .collect::<Vec<T>>()
    )
  }
  */

/*
  /// `execute` statement returning only the first item in the list
  fn execute_with_params_first<S, P>(&mut self, query: S, params: P) -> Result<Option<R>>
  where S: std::convert::AsRef<str>,
        P: Params,
  {
    let r = self.execute_with_params(query, params)?;
    Ok(r.into_iter().nth(0))
  }

  /// `execute` statement returning the first item in the list as an object of the given type `T`
  fn execute_with_params_first_try_as_object<S, P, T, E>(&mut self, query: S, params: P) -> Result<Option<T>>
  where S: std::convert::AsRef<str>,
        P: Params,
        T: for<'a> std::convert::TryFrom<&'a R, Error=E>,
        error::Error: std::convert::From<E>,
  {
    if let Some(r) = self.execute_with_params_first::<_, _>(query, params)? {
      Ok(Some((&r).try_into()?))
    } else {
      Ok(None)
    }
  }
  */

/*
  /// `execute` statement returning the first item in the list as an object of the given type `T`
  fn execute_with_params_first_as_object<S, P, T>(&mut self, query: S, params: P) -> Result<Option<T>>
  where S: std::convert::AsRef<str>,
        P: Params,
        T: for<'a> std::convert::From<&'a R>,
  {
    if let Some(r) = self.execute_with_params_first::<_, _>(query, params)? {
      Ok(Some((&r).into()))
    } else {
      Ok(None)
    }
  }
  */

/*
  /// `execute` statement dropping returned results
  fn execute_with_params_drop<S, P>(&mut self, query: S, params: P) -> Result<()>
  where S: std::convert::AsRef<str>,
        P: Params,
  {
    self.execute_with_params_first::<_, _>(query, params)?;
    Ok(())
  }
  */

  /// `query` statement returning list of objects of type `T`
  fn query_try_as_object<S, T>(&mut self, query: S) -> Result<Vec<T>>
  where S: std::convert::AsRef<str>,
        T: TryFromRefRow<R>,
  {
    self.query(query)?
    .iter()
    .map(|r: &R| Ok(T::try_from(r)?) )
    .collect::<Result<Vec<T>>>()
  }

/*
  /// `query` statement returning list of objects of type `T`
  fn query_as_object<S, T>(&mut self, query: S) -> Result<Vec<T>>
  where S: std::convert::AsRef<str>,
        T: for<'a> std::convert::From<&'a R>,
  {
    Ok(
    self.query(query)?
    .iter()
    .map(|r: &R| r.into() )
    .collect::<Vec<T>>()
    )
  }
  */

  /// `query` statement returning only the first item in the list
  fn query_first<S>(&mut self, query: S) -> Result<Option<R>>
  where S: std::convert::AsRef<str>,
  {
    Ok(self.query(query)?.into_iter().nth(0))
  }

  /// `query` statement returning only the first item as an object of type `T`
  fn query_first_try_as_object<S, T>(&mut self, query: S) -> Result<Option<T>>
  where S: std::convert::AsRef<str>,
        T: TryFromRefRow<R>,
  {
    if let Some(r) = self.query_first::<_>(query)? {
      Ok(Some(T::try_from(&r)?))
    } else {
      Ok(None)
    }
  }

/*
  /// `query` statement returning only the first item as an object of type `T`
  fn query_first_as_object<S, T>(&mut self, query: S) -> Result<Option<T>>
  where S: std::convert::AsRef<str>,
        T: for<'a> std::convert::From<&'a R>,
  {
    if let Some(r) = self.query_first::<_>(query)? {
      Ok(Some((&r).into()))
    } else {
      Ok(None)
    }
  }
  */

  /// `query` statement dropping returned results
  fn query_drop<S>(&mut self, query: S) -> Result<()>
  where S: std::convert::AsRef<str>
  {
    self.query_first::<_>(query)?;
    Ok(())
  }
}

mod sql;    // pub use sql::Sql;
mod table;  pub use table::{Table, TableStatement, TableFlavoredStatement};
mod insert; pub use insert::{Insert, InsertMultiple, InsertStatement, InsertFlavoredStatement};
mod select; pub use select::{Select as SelectV2, SelectStatement, SelectFlavoredStatement};
mod update; pub use update::{Update, UpdateStatement, UpdateFlavoredStatement};
mod delete; pub use delete::{Delete, DeleteStatement, DeleteFlavoredStatement};
mod filter; pub use filter::{Filter, FlavoredFilter};
mod order;  pub use order::{Order, FlavoredOrder};

/// Combine a flavored statement with optional filter, order, limit and offset to return full statement
fn statement_with_conn_filter_order_limit_offset_options<C, R, F, O>(statement: String, conn: &C, filter: Option<&F>, order: Option<&O>, limit: Option<usize>, offset: Option<usize>) -> Result<String>
where C: Connection<R>, R: Row, F: FlavoredFilter, O: FlavoredOrder,
{
  let statement = if let Some(filter) = filter { 
    let filter = filter.filter(conn)?;
    if ! filter.is_empty() { format!("{statement} WHERE {filter}") } 
    else { statement }
  } else { statement };
  let statement = if let Some(order) = order {
    let order = order.as_order_clause(conn)?;
    if ! order.is_empty() { format!("{statement} ORDER BY {order}") }
    else { statement }
  } else { statement };
  let statement = if let Some(limit) = limit { format!("{statement} LIMIT {limit}") } else { statement };
  let statement = if let Some(offset) = offset { 
    if offset > 0 { format!("{statement} OFFSET {offset}") }
    else { statement }
  } else { statement };
  Ok(statement)
}
/// Combine a statement with optional filter, order, limit and offset to return full statement
fn statement_with_filter_order_limit_offset_options<F, O>(statement: String, filter: Option<&F>, order: Option<&O>, limit: Option<usize>, offset: Option<usize>) -> Result<String>
where F: Filter, O: Order,
{
  let statement = if let Some(filter) = filter { 
    let filter = filter.filter();
    if ! filter.is_empty() { format!("{statement} WHERE {filter}") } 
    else { statement }
  } else { statement };
  let statement = if let Some(order) = order {
    let order = order.as_order_clause();
    if ! order.is_empty() { format!("{statement} ORDER BY {order}") }
    else { statement }
  } else { statement };
  let statement = if let Some(limit) = limit { format!("{statement} LIMIT {limit}") } else { statement };
  let statement = if let Some(offset) = offset { 
    if offset > 0 { format!("{statement} OFFSET {offset}") }
    else { statement }
  } else { statement };
  Ok(statement)
}

#[cfg(test)]
pub mod tests {
  use super::*;

  pub struct SQLiteFlavoredConnection {}
  impl<R> Connection<R> for SQLiteFlavoredConnection 
  where R: traits::Row
  {
    fn flavor(&self) -> Flavor { Flavor::SQLite }
    fn execute_with_params<S, P>(&mut self, _query: S, _params: &P) -> Result<()>
    where S: std::convert::AsRef<str>, P: Params,
    { Err("command not available for SQLiteFlavoredConnection".into()) }
    fn execute_with_params_iterator<'a, S, I, P>(&mut self, _query: S, _params_iter: I) -> Result<()>
    where S: std::convert::AsRef<str>, P: Params + 'a, 
          I: core::iter::IntoIterator<Item = &'a P>,
    { Err("command not available for SQLiteFlavoredConnection".into()) }
    fn query<S>(&mut self, _query: S) -> Result<Vec<R>>
    where S: std::convert::AsRef<str>,
    { Err("command not available for SQLiteFlavoredConnection".into()) }
  }

  pub struct Row {}
  impl row::Row for Row {
    fn get_value(&self, _i: usize) -> Option<Result<Value>> { None }
    fn get<T>(&self, _i: usize) -> Option<Result<T>>
    where T: row::TryFromValue,
    { None }
  }
}

#[cfg(feature="compatibility_v0_10")]
mod execute; 
#[cfg(feature="compatibility_v0_10")]
pub use execute::{ExecuteTrait};
#[cfg(feature="compatibility_v0_10")]
mod query; 
#[cfg(feature="compatibility_v0_10")]
pub use query::{QueryTrait};

// mod derive_sql; pub use derive_sql::{DeriveSqlTrait, Params, Param, ToParam, Row, FromValue, };
// mod select; pub use select::Select;
// mod connection; pub use connection::Connection;

#[cfg(feature="compatibility_v0_10")]
pub trait AsStatement<I> {
  fn as_statement(&self) -> Result<String>;
}

#[cfg(feature="compatibility_v0_10")]
pub trait IsSelect {}

#[cfg(feature="compatibility_v0_10")]
pub trait Select<T, I>
where T: AsStatement<I> + IsSelect
{
  fn select(&self, as_statement: &T) -> Result<Vec<I>>;
}

#[cfg(feature="compatibility_v0_10")]
pub trait CreateTable<T>
where T: AsStatement<()>
{
  fn create_table(&mut self, as_statement: &T) -> Result<()>;
}
