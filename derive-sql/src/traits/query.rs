use super::*;

/// Generic trait to be implemented by SQL drivers (or more likely by proxy to SQL drivers) so that the 
/// functionalities provided by this crate can be leveraged
pub trait QueryTrait<R>
where R: Row,
{

/*
  fn query_first<T, P, R, F>(&mut self, query: &str, params: P, f: F) -> Result<Option<T>>
  where P: Params,
        R: Row,
        F: FnOnce(&R) -> Result<T>;
        */

  fn query_row(&mut self, query: &str) -> Result<Vec<R>>;
  /*
  {
    self.query_row_with_params(query, ())
  }

  fn query_row_with_params<P>(&mut self, query: &str, params: P) -> Result<Vec<R>>
  where P: Params;
  */
}

/*
pub trait Row {
  fn get_string(&self, i: usize) -> Result<Option<String>>;
  fn get_i64(&self, i: usize)    -> Result<Option<i64>>;
}
*/

/*
struct Value<T> 
where T: FromValue
{
  value: T,
}

pub trait FromValue {
  fn as_string(&self) -> Option<String>;
  fn as_i64(&self)    -> Option<i64>;
}

#[cfg(attribute = "sqlite")]
impl FromValue for ::rusqlite::types::FromSql {
*/
  


/*
pub struct Value<T> 
where T: FromValue,
{
  t: T,
}


pub trait FromValue<T> {
  fn from(t: T) -> Result<Self>;
}

#[cfg(feature = "sqlite")]
impl FromValue<T: rusqlite::types::ValueRef<'_>> for String { 
  fn from(t: T) -> Result<Self> { Ok(rusqlite::types::FromSql::column_result(t)?) }
}
*/

