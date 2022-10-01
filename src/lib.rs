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
//!
//! let conn = rusqlite::Connection::open_in_memory().unwrap();
//!
//! let person_sql = PersonSql::from_rusqlite(&conn).unwrap();
//!
//! // Create Table in SQL database
//! person_sql.create_table().unwrap();
//!
//! // Insert person into SQL database
//! let person = Person { name: "Jo".to_string(), age: 24 };
//! person_sql.insert(&person).unwrap();
//!
//! // Retrieve list of persons from SQL database
//! let persons: Vec<Person> = person_sql.select().unwrap();
//! assert!(persons.len() == 1);
//! assert!(persons[0].name.eq("Jo"));
//!
//! // Insert Jane
//! let jane = Person { name: "Jane".to_string(), age: 27 };
//! person_sql.insert(&jane).unwrap();
//!
//! // Check Jane's age
//! let p: Person = person_sql.select().unwrap()
//!             .into_iter().find(|p| p.name.eq("Jane")).unwrap();
//! assert!(p.age == 27);
//!
//! // Update Jane
//! let jane: Person = person_sql.select().unwrap()
//!             .into_iter().find(|p| p.name.eq("Jane")).unwrap();
//! let update_to_jane = Person { name: jane.name.clone(), age: jane.age+1 };
//! person_sql.update_to(&jane, &update_to_jane).unwrap();
//! let updated_jane: Person = person_sql.select().unwrap()
//!             .into_iter().find(|p| p.name.eq("Jane")).unwrap();
//! assert!(updated_jane.age == 28);
//!
//! // Check Jane's age
//! let p: Person = person_sql.select().unwrap()
//!             .into_iter().find(|p| p.name.eq("Jane")).unwrap();
//! assert!(p.age == 28);
//!
//! // Delete Jo
//! let jo: Person = person_sql.select().unwrap()
//!             .into_iter().find(|p| p.name.eq("Jo")).unwrap();
//! person_sql.delete(&jo).unwrap();
//! let jo: Option<Person> = person_sql.select().unwrap()
//!             .into_iter().find(|p| p.name.eq("Jo"));
//! assert!(jo.is_none());
//!
//! // Check that database only contains Jane
//! let persons: Vec<Person> = person_sql.select().unwrap();
//! assert!(persons.len() == 1);
//! assert!(persons[0].name.eq("Jane"));
//!
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

