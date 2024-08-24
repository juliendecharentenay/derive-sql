//! The crate `derive_sql` is articulated around:
//! - traits that allow SQL database drivers (`rusqlite` and `mysql`) to provide
//!   defined set of functionalities;
//! - traits that scaffold on these common functionalities to wrap common
//!   SQL statement functions into typed interfaces.
//!
//! The trait `Connection` provides a uniform interface for interaction with SQL
//! database drivers via `query` calls where all elements of the query are encompassed
//! in the statement; and `execute` calls where the statement refers to parameters
//! to be provided.
//!
//! ```rust
//! use derive_sql::traits;
//!
//! // Function to create table, populate table with an entry, and retrieve the first entry field name
//! fn run<S, R>(s: &mut S) -> String 
//! where S: traits::Connection<R>,
//!       R: traits::Row,
//! {
//!   s.query_drop("DROP TABLE IF EXISTS example_table").unwrap();
//!
//!   s.query_drop("CREATE TABLE example_table (
//!     id INTEGER,
//!     name TEXT
//!   )").unwrap(); 
//!   
//!   s.execute_with_params("INSERT INTO example_table (id, name) VALUES (?, ?)",
//!     &(1i64, "Jane Doe".to_string())
//!   ).unwrap();
//!   
//!   let r: String = s.query_first("SELECT name FROM example_table").unwrap()
//!   .map(|r| r.get(0).unwrap().unwrap())
//!   .unwrap();
//!
//!   r
//! }
//!   
//! // Called using a SQLite connection via rusqlite
#![cfg_attr(not(feature="sqlite"), doc = "/*")]
//! let mut conn = rusqlite::Connection::open_in_memory().unwrap();
//! assert!(run(&mut conn).eq("Jane Doe"));
#![cfg_attr(not(feature="sqlite"), doc = "*/")]
//!
//! // Call using a MySQL connection
#![cfg_attr(not(feature="mysql"), doc = "/*")]
//! let mut mysql_conn = mysql::Conn::new(
//!   mysql::Opts::from_url("mysql://test@localhost/simpledb").unwrap()
//! ).unwrap();
//! assert!(run(&mut mysql_conn).eq("Jane Doe"));
#![cfg_attr(not(feature="mysql"), doc = "*/")]
//! ```
//!
//! The following is showing the same test using a set of traits provided to standardize the interaction with SQL
//! databases:
//!
//! ```rust
//! use derive_sql::traits::{self, Table, TableStatement, SelectV2, SelectStatement, Insert, InsertStatement,
//!   Delete, DeleteStatement, 
//!   Update, UpdateStatement, 
//!   Param, Params, Row, TryFromRefRow,
//!   ToParam,
//! };
//! use derive_sql::structs::{filter, Field};
//! use derive_sql::Result;
//!
//! // Define a `struct` representing the data to be stored
//! /* #[derive(Clone)] */
//! struct People {
//!   id: i64,
//!   name: String,
//! }
//!
//! // Implement traits to convert `People` to and from
//! impl Params for People {
//!   fn as_vec_params(&self) -> Result<Vec<Param>> {
//!     Ok(vec![self.id.to_param()?, self.name.to_param()?])
//!   }
//! }
//! impl<R> TryFromRefRow<R> for People 
//! where R: Row,
//! {
//!   fn try_from(r: &R) -> Result<Self> {
//!     Ok(People {
//!       id: r.get(0).ok_or(derive_sql::Error::RowItemNotFound(0))??,
//!       name: r.get(1).ok_or(derive_sql::Error::RowItemNotFound(1))??,
//!     })
//!   }
//! }
//!
//! #[derive(Default)]
//! struct SqlPeople {}
//!
//! // Implement traits to manipulate the data using struct `SqlPeople`
//! impl TableStatement for SqlPeople {
//!   fn create_stmt(&self)              -> Result<String> { Ok(format!("CREATE TABLE example2_table ( id INTEGER, name TEXT )")) }
//!   fn create_if_not_exist_stmt(&self) -> Result<String> { Ok(format!("CREATE TABLE IF NOT EXISTS example2_table ( id INTEGER, name TEXT )")) }
//!   fn drop_stmt(&self)                -> Result<String> { Ok(format!("DROP TABLE IF EXISTS example2_table")) }
//! }
//! impl SelectStatement for SqlPeople {
//!   fn select_stmt(&self) -> Result<String> { Ok(format!("SELECT id,name FROM example2_table")) }
//! }
//! impl InsertStatement for SqlPeople {
//!   fn insert_stmt(&self) -> Result<String> { Ok(format!("INSERT INTO example2_table (id, name) VALUES (?, ?)")) }
//! }
//! impl UpdateStatement for SqlPeople {
//!   fn update_stmt(&self) -> Result<String> { Ok(format!("UPDATE example2_table SET `id` = ?, `name` = ?")) }
//! }
//! impl DeleteStatement for SqlPeople {
//!   fn delete_stmt(&self) -> Result<String> { Ok(format!("DELETE FROM example2_table")) }
//! }
//!
//! fn run<S, R>(s: &mut S) -> String
//! where S: traits::Connection<R>,
//!       R: traits::Row,
//! {
//!   SqlPeople::default().drop(s).unwrap();
//!   SqlPeople::default().create(s).unwrap();
//!   SqlPeople::default().insert(s, &People { id: 1, name: "Jane Doe".to_string() }).unwrap();
//!   SqlPeople::default().insert(s, &People { id: 2, name: "Jane Foe".to_string() }).unwrap();
//!   SqlPeople::default().insert(s, &People { id: 3, name: "Jane Goe".to_string() }).unwrap();
//!   let r: Vec<People> = SqlPeople::default().select(s).unwrap();
//!   assert!(r.len() == 3);
//!
//!   let r: Vec<People> = SqlPeople::default().select_with_filter(s, 
//!     &filter::Or::from((Field::from("id").eq(1), Field::from("id").eq(3)))
//!   ).unwrap();
//!   assert!(r.len() == 2);
//!
//!   let r: Vec<People> = SqlPeople::default().select_with_filter(s, 
//!     &filter::And::from((Field::from("id").eq(1), Field::from("name").ne("Jane Doe")))
//!   ).unwrap();
//!   assert!(r.len() == 0);
//!
//!   let r: Vec<People> = SqlPeople::default().select_with_filter_order_limit_offset(s,
//!     &filter::None::default(),
//!     &Field::from("id").descending(),
//!     1, // Limit
//!     0, // Offset
//!   ).unwrap();
//!   assert!(r[0].id == 3);
//!
//!   SqlPeople::default().update_with_filter(s,
//!     &Field::from("id").eq(3),
//!     &People { id: 3, name: "Francis".to_string() },
//!   ).unwrap();
//!   let r: Vec<People> = SqlPeople::default().select_with_filter(s,
//!     &Field::from("id").eq(3),
//!   ).unwrap();
//!   assert!(r[0].name.eq("Francis"));
//!
//!   SqlPeople::default().delete_with_filter(s, &Field::from("name").eq("Francis")).unwrap();
//!   let r: Vec<People> = SqlPeople::default().select(s).unwrap();
//!   assert!(r.len() == 2);
//!
//!   let r: Vec<People> = SqlPeople::default().select_with_filter(s, &Field::from("id").eq(1)).unwrap();
//!   assert!(r.len() == 1);
//!
//!   r[0].name.clone()
//! }
//!
//! // Called using a SQLite connection via rusqlite
#![cfg_attr(not(feature="sqlite"), doc = "/*")]
//! let mut conn = rusqlite::Connection::open_in_memory().unwrap();
//! assert!(run(&mut conn).eq("Jane Doe"));
#![cfg_attr(not(feature="sqlite"), doc = "*/")]
//!
//! // Call using a MySQL connection
#![cfg_attr(not(feature="mysql"), doc = "/*")]
//! let mut mysql_conn = mysql::Conn::new(
//!   mysql::Opts::from_url("mysql://test@localhost/simpledb").unwrap()
//! ).unwrap();
//! assert!(run(&mut mysql_conn).eq("Jane Doe"));
#![cfg_attr(not(feature="mysql"), doc = "*/")]
//! ```
//!
//! This replicates the same test using the derive macro `DeriveSqlStatement`:
//!
//! ```rust
//! use derive_sql::{DeriveSqlStatement};
//! use derive_sql::traits::{Table, Insert, Delete, Update, SelectV2};
//! use derive_sql::{traits, structs::filter, structs::Field, Result};
//!
//! // Define a `struct` representing the data to be stored
//! #[derive(DeriveSqlStatement)]
//! struct People {
//!   id: i64,
//!   name: String,
//! }
//!
//! fn run<S, R>(s: &mut S) -> String
//! where S: traits::Connection<R>,
//!       R: traits::Row,
//! {
//!   SqlPeople::default().drop(s).unwrap();
//!   SqlPeople::default().create(s).unwrap();
//!   SqlPeople::default().insert(s, &People { id: 1, name: "Jane Doe".to_string() }).unwrap();
//!   SqlPeople::default().insert(s, &People { id: 2, name: "Jane Foe".to_string() }).unwrap();
//!   SqlPeople::default().insert(s, &People { id: 3, name: "Jane Goe".to_string() }).unwrap();
//!   let r: Vec<People> = SqlPeople::default().select(s).unwrap();
//!   assert!(r.len() == 3);
//!
//!   let r: Vec<People> = SqlPeople::default().select_with_filter(s, 
//!     &filter::Or::from((Field::from("id").eq(1), Field::from("id").eq(3)))
//!   ).unwrap();
//!   assert!(r.len() == 2);
//!
//!   let r: Vec<People> = SqlPeople::default().select_with_filter(s, 
//!     &filter::And::from((Field::from("id").eq(1), Field::from("name").ne("Jane Doe")))
//!   ).unwrap();
//!   assert!(r.len() == 0);
//!
//!   let r: Vec<People> = SqlPeople::default().select_with_filter_order_limit_offset(s,
//!     &filter::None::default(),
//!     &Field::from("id").descending(),
//!     1, // Limit
//!     0, // Offset
//!   ).unwrap();
//!   assert!(r[0].id == 3);
//!
//!   SqlPeople::default().update_with_filter(s,
//!     &Field::from("id").eq(3),
//!     &People { id: 3, name: "Francis".to_string() },
//!   ).unwrap();
//!   let r: Vec<People> = SqlPeople::default().select_with_filter(s,
//!     &Field::from("id").eq(3),
//!   ).unwrap();
//!   assert!(r[0].name.eq("Francis"));
//!
//!   SqlPeople::default().delete_with_filter(s, &Field::from("name").eq("Francis")).unwrap();
//!   let r: Vec<People> = SqlPeople::default().select(s).unwrap();
//!   assert!(r.len() == 2);
//!
//!   let r: Vec<People> = SqlPeople::default().select_with_filter(s, &Field::from("id").eq(1)).unwrap();
//!   assert!(r.len() == 1);
//!
//!   r[0].name.clone()
//! }
//!
//! // Called using a SQLite connection via rusqlite
#![cfg_attr(not(feature="sqlite"), doc = "/*")]
//! let mut conn = rusqlite::Connection::open_in_memory().unwrap();
//! assert!(run(&mut conn).eq("Jane Doe"));
#![cfg_attr(not(feature="sqlite"), doc = "*/")]
//!
//! // Call using a MySQL connection
#![cfg_attr(not(feature="mysql"), doc = "/*")]
//! let mut mysql_conn = mysql::Conn::new(
//!   mysql::Opts::from_url("mysql://test@localhost/simpledb").unwrap()
//! ).unwrap();
//! assert!(run(&mut mysql_conn).eq("Jane Doe"));
#![cfg_attr(not(feature="mysql"), doc = "*/")]
//!
//! ```
//!
//! ## Legacy v0.10 feature:
//!
//! Available by activating feature `compability_v0_10`
//!
//! The trait `Sqlable` that defines a set of operation for 
//! interacting with SQL tables:
//! - `count` to provide a count of the number of items in the table.
//! - `select` to return an array of the items in the table.
//! - `insert` to insert a new item in the table.
//! - `update` to update an existing item(s) with the values of the provided item.
//! - `delete` to delete items in the table.
//! - `delete_table` to drop the table.
//!
//! Implementation of the trait should allow the user of the trait to interact with the table via the above
//! interface...
//!
//! The trait `Selectable` provides a possible interface for selector queries
//! used in the `Sqlable` trait. It is a possible option - but not limited
//! to it as the `Sqlable` trait uses an associated type for `Selector`.
//!
//! This crate includes:
//! - the derive macro `DeriveSqlite` [when compiled with the feature `--features sqlite`]
//! which provides an implementation of the `Sqlable` trait for SQLite as a wrapper around the `rusqlite`
//! crate;
//! - the derive macro `DeriveMysql` [when compiled with the feature `--features mysql`]
//! which provides an implementation of the `Sqlable` trait for MySQL as a wrapper around the `mysql`
//! crate;
//!
//! Please see examples here and the `DeriveSqlite` documentation.
//!
//! # Features:
//! - `sqlite` provides a derive macro that implements the `Sqlable` trait for SQLite database (implemented as a wrapper around the `rusqlite` crate);
//! - `mysql` provides a derive macro that implements the `Sqlable` trait for MySQL database (implemented as a wrapper around the `mysql` crate);
//!
//! # Mocking:
//! The example of code below shows how the trait can be mocked using `mockall` for unit testing purposes. The
//! example uses `mockall` external trait functionality - ie works in a code using this crate as a dependency.
//! Note: one has to explicitely nominates the associated type in the method definitions.
//!
#![cfg_attr(feature="compatibility_v0_10", doc = "```rust")]
#![cfg_attr(not(feature="compatibility_v0_10"), doc = "```ignore")]
//! mockall::mock! {
//!   SqlableStruct {}
//!   impl derive_sql::Sqlable for SqlableStruct {
//!     type Item = String;
//!     type Error = Box<dyn std::error::Error>;
//!     type Selector = ();
//!     
//!     fn count(&self, s: ()) -> Result<usize, Box<dyn std::error::Error>>;
//!     fn select(&self, s: ()) -> Result<Vec<String>, Box<dyn std::error::Error>>;
//!     fn insert(&mut self, item: String) -> Result<String, Box<dyn std::error::Error>>;
//!     fn update(&mut self, s: (), item: String) -> Result<String, Box<dyn std::error::Error>>;
//!     fn delete(&mut self, s: ()) -> Result<(), Box<dyn std::error::Error>>;
//!     fn delete_table(&mut self) -> Result<(), Box<dyn std::error::Error>>;
//!   }
//! }
//!
//! fn my_function<S>(s: &mut S) -> Result<usize, Box<dyn std::error::Error>> 
//! where S: derive_sql::Sqlable<Selector = (), Item = String, Error = Box<dyn std::error::Error>>,
//! {
//!   let _ = s.insert("an item".to_string())?;
//!   Ok(s.count(())?)
//! }
//!
//! // Create mock
//! let mut mock = MockSqlableStruct::new();
//! // Configure mock
//! mock.expect_insert()
//! .with(mockall::predicate::eq("an item".to_string()))
//! .returning(|s| Ok(s));
//! mock.expect_count().returning(|_| Ok(11));
//!
//! // Check result
//! assert!(matches!(my_function(&mut mock), Ok(11)));
//!
//! ```
//! 
//!

#[cfg(feature="sqlite")]
/// Re-export `rusqlite` library used
pub use rusqlite;

#[cfg(feature="mysql")]
/// Re-export `mysql` library used
pub use mysql;

pub mod traits;
pub mod proxy;
pub mod structs; // pub use structs::{Field, filter, order};

#[cfg(feature="compatibility_v0_10")]
mod sqlable; 
#[cfg(feature="compatibility_v0_10")]
pub use sqlable::Sqlable;

#[cfg(feature="compatibility_v0_10")]
pub mod generics;

#[cfg(feature="compatibility_v0_10")]
mod selectable; 

#[cfg(feature="compatibility_v0_10")]
/// Implementation of generic approach to `WHERE` clauses filtering. Provides a generic operator for single clause and 
/// `And` and `Or` clauses combinator;
pub use selectable::filter;

#[cfg(feature="compatibility_v0_10")]
pub use selectable::Selectable;

#[cfg(feature="compatibility_v0_10")]
/// Convenient struct for implementing a simple filter, ie a struct that generates the content of a simple `WHERE a = value` clause
pub use selectable::SimpleFilter;

#[cfg(feature="compatibility_v0_10")]
/// Convenient struct for implementing a limit, ie a struct that generates the content of a `LIMIT value` clause
pub use selectable::SimpleLimit;

#[cfg(feature="compatibility_v0_10")]
/// Convenient struct for implementing an offset, ie a struct that generates the content of an `OFFSET value` clause
pub use selectable::SimpleOffset;

#[cfg(feature="compatibility_v0_10")]
/// Convenient struct for implementing an order by, ie a struct that generates the content of an `ORDER BY value ASC|DESC` clause
pub use selectable::{SimpleOrder, Order};

#[cfg(all(feature="sqlite", feature="compatibility_v0_10"))]
/// Derive macro to implement the `Sqlable` trait for a struct with named fields so that instances of the struct
/// can be saved, queried, stored to/from an SQLite database. Uses `rusqlite`. Requires `--features sqlite`.
pub use derive_sql_sqlite::DeriveSqlite;

#[cfg(all(feature="mysql", feature="compatibility_v0_10"))]
/// Derive macro to implement the `Sqlable` trait for a struct with named fields so that instances of the struct
/// can be saved, queried, stored to/from a MySQL database. Uses `mysql`. Requires `--features mysql`.
pub use derive_sql_mysql::DeriveMysql;

pub use derive_sql_statement::DeriveSqlStatement;

mod error;
pub use error::{Result, DeriveSqlResult, Error};
