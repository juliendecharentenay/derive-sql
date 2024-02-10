//! Procedural macro to automatically generate an `Sqlable` trait for the 
//! provided struct that is compatible with an MySQL database connected using the `mysql` crate
//! <https://crates.io/crates/mysql>.
//!
//! # How to use
//! 
//! You write:
//! ```rust
//! # use derive_sql::*;
//! # use derive_sql_mysql::DeriveMysql;
//! #[derive(DeriveMysql)]
//! pub struct Person {
//!   name: String,
//!   age: u32,
//! }
//! ```
//! 
//! And you can use:
//! ```rust
//! # use derive_sql::*;
//! # use derive_sql_mysql::DeriveMysql;
//! # #[derive(DeriveMysql)]
//! # pub struct Person {
//! #   name: String,
//! #   age: u32,
//! # }
//!
//! let pool = mysql::Pool::new("mysql://test@localhost/simpledb").unwrap();
//! let mut connection = pool.get_conn().unwrap();
//! let mut db: PersonMysql<_> = connection.into();
//!
//! // initialise
//! db.create_table().unwrap();
//!
//! // Insert entries
//! db.insert(Person {name: "Abi".to_string(), age: 31 }).unwrap();
//! db.insert(Person {name: "Bert".to_string(), age: 32 }).unwrap();
//! db.insert(Person {name: "Charlie".to_string(), age: 33 }).unwrap();
//! 
//! // Query
//! let persons: Vec<Person> = db.select(Box::new(SimpleFilter::try_from(("age", 32)).unwrap())).unwrap();
//! assert!(persons[0].name.eq("Bert"));
//!
//! // Update
//! db.update(Box::new(SimpleFilter::try_from(("name", "Abi")).unwrap()), Person { name: "Abi".to_string(), age: 32 }).unwrap();
//!
//! // Delete
//! db.delete(Box::new(SimpleFilter::try_from(("name", "Abi")).unwrap())).unwrap();
//!
//! // Clear the table
//! db.delete_table().unwrap();
//! ```
//!
//! # Container attributes:
//! - `#[derive_sqlite(ident = ...)]` overwrite the name of the `rusqlite` wrapper from `{class}Mysql`;
//! - `#[derive_sqlite(table_name = "...")]` specify the name of the table (default to the container name in lower case);
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
//!
//! - `#[derive_sqlite(on_insert = ...)]` nominate a function of the type `fn() -> {type}` with `{type}` corresponding to the type of the 
//! field. The function is called when the item is inserted and the value returned by the function is assigned to the field before the
//! item is inserted. Typical use is to assign a creation date.
//! - `#[derive_sqlite(on_update = ...)]` nominate a function of the type `fn() -> {type}` with `{type}` corresponding to the type of the 
//! field. The function is called when the item is updated and the value returned by the function is assigned to the field before the
//! item is updated. Typical use is to assign a last modified date.
//!

mod mysql;

use attribute_derive::{Attribute};

#[derive(Attribute)]
#[attribute(ident = derive_sqlite)]
struct Attrs {
  ident: Option<syn::Ident>,
  table_name: Option<String>,
}

/*
#[derive(Attribute)]
#[attribute(ident = derive_sqlite)]
struct FieldAttrs {
  #[attribute(default = false)]
  is_primary_key: bool,
  on_insert: Option<syn::PatPath>,
  on_update: Option<syn::PatPath>,
}
*/

#[proc_macro_derive(DeriveMysql, attributes(derive_sqlite))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  syn::parse(input)
  .and_then(|ast: syn::DeriveInput| {
    Ok(mysql::Mysql::try_from(&ast)?.generate()?)
  })
  .unwrap_or_else(|e| e.into_compile_error())
  .into()
}

