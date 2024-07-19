//! Procedural macro to automatically generate SQL statements traits `TableStatement`, `SelectStatement`, `InsertStatement`, `DeleteStatement` for the 
//! provided struct as well as conversion to SQL parameters and from SQL rows.
//!
//! # How to use
//! 
//! You write:
//! ```rust
//! # use derive_sql::*;
//! #[derive(derive_sql::DeriveSqlStatement)]
//! pub struct Person {
//!   name: String,
//!   age: u32,
//! }
//! ```
//! 
//! And you can use:
//! ```rust
//! # use derive_sql::*;
//! # #[derive(DeriveSqlStatement)]
//! # pub struct Person {
//! #   name: String,
//! #   age: u32,
//! # }
//! use derive_sql::structs::*;
//!
//! fn handle<Con, Row>(s: &mut Con) 
//! where Con: traits::Connection<Row>,
//!       Row: traits::Row,
//! {
//!   use derive_sql::traits::{Table, SelectV2, Insert, Delete, Update};
//!   let db = SqlPerson::default();
//!
//!   // initialise
//!   db.create(s).unwrap();
//!
//!   // Insert entries
//!   db.insert(s, &Person {name: "Abi".to_string(), age: 31 }).unwrap();
//!   db.insert(s, &Person {name: "Bert".to_string(), age: 32 }).unwrap();
//!   db.insert(s, &Person {name: "Charlie".to_string(), age: 33 }).unwrap();
//! 
//!   // Query
//!   let persons: Vec<Person> = db.select_with_filter(s, &Field::from("age").eq(32)).unwrap();
//!   assert!(persons[0].name.eq("Bert"));
//!
//!   // Update
//!   db.update_with_filter(s, &Field::from("name").eq("Abi"), &Person { name: "Abi".to_string(), age: 32 }).unwrap();
//!
//!   // Delete
//!   db.delete_with_filter(s, &Field::from("name").eq("Abi")).unwrap();
//!
//!   // Clear the table
//!   db.drop(s).unwrap();
//! }
//!
//! let pool = ::mysql::Pool::new("mysql://test@localhost/simpledb").unwrap();
//! let mut connection = pool.get_conn().unwrap();
//! handle(&mut connection);
//! ```
//!
//! # Container attributes:
//! - `#[derive_sqlite(ident = ...)]` overwrite the name of the wrapper from `Sql{class}`;
//! - `#[derive_sqlite(table_name = "...")]` specify the name of the table (default to the container name in lower case);
//! - `#[derive_sqlite(read_only = true/false)]` specify whether to implement read/write (ie table, select, insert, update, delete, to params conversion and from row conversion)
//!    or read only statements (ie select and from row conversion)
//!
//! # Field attributes:
//! - `#[derive_sqlite(is_primary_key = true)]` nominate that one of the field is a primary key. Only one primary key can be specified.
//! primary key fields are unique in the table. Primary key can NOT be a String - the following will not compile:
//!
//! ```compile_fail
//! # use derive_sql::*;
//! # use derive_sql_mysql::DeriveMysql;
//! #[derive(DeriveMysql)]
//! pub struct Person {
//!   #[derive_sqlite(is_primary_key = true)]
//!   name: String,
//!   age: u32,
//! }
//! ```
//!

mod statement;

use attribute_derive::{Attribute};

#[derive(Attribute)]
#[attribute(ident = derive_sql)]
struct Attrs {
  ident: Option<syn::Ident>,
  table_name: Option<String>,
  read_only: bool,
}

#[proc_macro_derive(DeriveSqlStatement, attributes(derive_sql, derive_sqlite))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  syn::parse(input)
  .and_then(|ast: syn::DeriveInput| {
    Ok(statement::Statement::try_from(&ast)?.generate()?)
  })
  .unwrap_or_else(|e| e.into_compile_error())
  .into()
}

