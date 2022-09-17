//! Derive methods to store a struct in an SQL database
//!
//! This crate implements methods to faciliate the storage, retrieval
//! query and deletion of structured object in an SQL database. The
//! object is structured according to the `struct` fields.
//!
//! # Quick start
//!
//! Add 'derive_sql' as a dependency to your 'Cargo.toml'.
//!
//! ## What you write
//!
//! ```rust
//! use derive_sql::DeriveSql;
//! 
//! #[derive(DeriveSql)]
//! struct Person {
//!   name: String,
//!   age: u32,
//! }
//! ```
//!
//! ## What you can use
//!
//! ```rust
//! # use rusqlite;
//! # use derive_sql::DeriveSql;
//! # #[derive(DeriveSql)]
//! # struct Person {
//! #   name: String,
//! #   age: u32,
//! # }
//! #
//! # let conn = rusqlite::Connection::open_in_memory().unwrap();
//!
//! // Create Table in SQL database
//! Person::create_table(&conn).unwrap();
//!
//! // Insert person into SQL database
//! let person = Person { name: "Jo".to_string(), age: 44 };
//! person.insert(&conn).unwrap();
//!
//! // Retrieve list of persons from SQL database
//! let persons: Vec<Person> = Person::select(&conn).unwrap();
//! assert!(persons.len() == 1);
//! assert!(persons[0].name.eq("Jo"));
//! ```

use proc_macro;
use syn;

mod sqltype;
mod implderive;

use sqltype::SqlType;
use implderive::ImplDerive;

#[proc_macro_derive(DeriveSql)]
pub fn derive_sql(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let ast = syn::parse(input).unwrap();
  ImplDerive { ast: &ast }.generate().unwrap().into()
}

