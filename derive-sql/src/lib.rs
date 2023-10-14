//! The crate `derive_sql` is articulated around two traits.
//!
//! The trait `Sqlable` that defines a set of operation for 
//! interacting with SQL tables:
//! - `create_table` to create the table;
//! - `count` to provide a count of the number of items in the table;
//! - `select` to return an array of the items in the table;
//! - `insert` to insert a new item in the table;
//! - `update` to update an existing item(s) with the values of the provided item;
//! - `delete` to delete items in the table.
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

mod sqlable; pub use sqlable::Sqlable;
mod selectable; pub use selectable::{Selectable, SimpleFilter, SimpleLimit, SimpleOffset};

#[cfg(feature="sqlite")]
/// Derive macro to implement the `Sqlable` trait for a struct with named fields so that instances of the struct
/// can be saved, queried, stored to/from an SQLite database. Uses `rusqlite`. Requires `--features sqlite`.
pub use derive_sqlite::DeriveSqlite;

