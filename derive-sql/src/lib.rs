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
//!  - `with-mock` provide a `MockSqlable` implementation of the `Sqlable` trait to use in testing. 
//!

mod sqlable; pub use sqlable::Sqlable;
mod selectable; 

pub use selectable::Selectable;

/// Convenient struct for implementing a simple filter, ie a struct that generates the content of a simple `WHERE a = value` clause
pub use selectable::SimpleFilter;

/// Convenient struct for implementing a limit, ie a struct that generates the content of a `LIMIT value` clause
pub use selectable::SimpleLimit;

/// Convenient struct for implementing am offset, ie a struct that generates the content of an `OFFSET value` clause
pub use selectable::SimpleOffset;

#[cfg(feature="sqlite")]
/// Derive macro to implement the `Sqlable` trait for a struct with named fields so that instances of the struct
/// can be saved, queried, stored to/from an SQLite database. Uses `rusqlite`. Requires `--features sqlite`.
pub use derive_sqlite::DeriveSqlite;

#[cfg(feature="with-mock")]
mod mock_sqlable; 

#[cfg(feature="with-mock")]
/// Convenience testing struct implementing `Sqlable` trait. Requires `--features with-mock`.
pub use mock_sqlable::MockSqlable;


