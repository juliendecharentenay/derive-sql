//! Proxy to manipulate SQL queries
use super::*;

mod log; pub use log::Log;

#[cfg(feature="sqlite")]
pub mod sqlite;

#[cfg(feature="mysql")]
pub mod mysql;

#[cfg(feature="postgres")]
pub mod postgres;

#[cfg(test)]
pub mod proxy_test {
  use super::*;

  pub fn run_connection<S, R>(s: &mut S) -> Result<()>
  where S: traits::Connection<R>,
        R: traits::Row,
  {
    s.query_drop("DROP TABLE IF EXISTS mytable_proxy_conn")?;

    s.query_drop("CREATE TABLE mytable_proxy_conn (
      id INTEGER,
      name TEXT
    )")?;

    s.execute_with_params("INSERT INTO mytable_proxy_conn (id, name) VALUES (?, ?)",
      &(1i64, "my name".to_string())
    )?;

    let r: String = s.query_first("SELECT name FROM mytable_proxy_conn")?
    .map(|r| Ok::<_, Error>(r.get::<String>(0).ok_or(Error::ResultConversionFail("String".to_string()))??.clone()))
    .ok_or(Error::QueryReturnNoResult)??;
    assert!(r.eq("my name"));

    Ok(())
  }

  pub fn run_with_date<S, R>(s: &mut S) -> Result<()>
  where S: traits::Connection<R>,
        R: traits::Row,
  {
    use chrono::Datelike;

    s.query_drop("DROP TABLE IF EXISTS run_with_date")?;

    s.query_drop("CREATE TABLE run_with_date (
      date DATE
    )")?;

    s.execute_with_params("INSERT INTO run_with_date (date) VALUES (?)",
      &chrono::naive::NaiveDate::from_ymd_opt(2024, 1, 2).ok_or("Invalid date")?,
    )?;

    let r: chrono::naive::NaiveDate = s.query_first("SELECT * FROM run_with_date")?
    .map(|r| Ok::<_, Error>(r.get::<chrono::naive::NaiveDate>(0).ok_or(Error::ResultConversionFail("NaiveDate".to_string()))??))
    .ok_or(Error::QueryReturnNoResult)??;
    assert!(r.year() == 2024);
    assert!(r.month() == 1);
    assert!(r.day() == 2);

    Ok(())
  }
}
