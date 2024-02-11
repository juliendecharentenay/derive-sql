//! The crate `derive_sql` is articulated around two traits.
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
//! This crate includes the derive macro `DeriveSqlite` [when compiled with the feature `--features sqlite`]
//! which provides an implementation of the `Sqlable` trait for SQLite as a wrapper around the `rusqlite`
//! crate.
//!
//! Please see examples here and the `DeriveSqlite` documentation.
//!
//! # Features:
//! - `sqlite` provide a derive macro to implement the `Sqlable` trait for SQLite database (implemented as a wrapper around the `rusqlite` crate);
//!
//! # Mocking:
//! The example of code below shows how the trait can be mocked using `mockall` for unit testing purposes. The
//! example uses `mockall` external trait functionality - ie works in a code using this crate as a dependency.
//! Note: one has to explicitely nominates the associated type in the method definitions.
//!
//! ```rust
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

mod sqlable; pub use sqlable::Sqlable;
mod selectable; 
mod middleware; 
#[cfg(feature="sqlite")]
pub use middleware::sqlite;

pub use selectable::Selectable;

/// Convenient struct for implementing a simple filter, ie a struct that generates the content of a simple `WHERE a = value` clause
pub use selectable::SimpleFilter;

/// Convenient struct for implementing a limit, ie a struct that generates the content of a `LIMIT value` clause
pub use selectable::SimpleLimit;

/// Convenient struct for implementing an offset, ie a struct that generates the content of an `OFFSET value` clause
pub use selectable::SimpleOffset;

/// Convenient struct for implementing an order by, ie a struct that generates the content of an `ORDER BY value ASC|DESC` clause
pub use selectable::{SimpleOrder, Order};

#[cfg(feature="sqlite")]
/// Derive macro to implement the `Sqlable` trait for a struct with named fields so that instances of the struct
/// can be saved, queried, stored to/from an SQLite database. Uses `rusqlite`. Requires `--features sqlite`.
pub use derive_sql_sqlite::DeriveSqlite;

#[cfg(feature="mysql")]
/// Derive macro to implement the `Sqlable` trait for a struct with named fields so that instances of the struct
/// can be saved, queried, stored to/from a MySQL database. Uses `mysql`. Requires `--features mysql`.
pub use derive_sql_mysql::DeriveMysql;

