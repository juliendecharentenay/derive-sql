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
//!   checked: bool,
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
//! #   checked: bool,
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
//! let person = Person { name: "Jo".to_string(), age: 24, checked: false };
//! person_sql.insert(&person).unwrap();
//!
//! // Retrieve list of persons from SQL database
//! assert!(person_sql.count_all().unwrap() == 1);
//! let persons: Vec<Person> = person_sql.select_all().unwrap();
//! assert!(persons.len() == 1);
//! assert!(persons[0].name.eq("Jo"));
//!
//! // Insert Jane
//! let jane = Person { name: "Jane".to_string(), age: 27, checked: false };
//! person_sql.insert(&jane).unwrap();
//!
//! // Check Jane's age
//! let select = SelectBuilder::default().set_filter(Filter::NameEqual("Jane".to_string())).build();
//! let p: Person = person_sql.select(select).unwrap().into_iter().nth(0).unwrap();
//! assert!(p.age == 27);
//! // or
//! let filter = Filter::And(Box::new(Filter::NameEqual("Jane".to_string())), Box::new(Filter::AgeEqual(27)));
//! assert!(person_sql.count(filter.into()).unwrap() == 1);
//!
//! // Update Jane
//! let filter = Filter::NameEqual("Jane".to_string());
//! let jane: Person = person_sql.select_one(filter.into()).unwrap().unwrap();
//! let update_to_jane = Person { name: jane.name.clone(), age: jane.age+1, checked: jane.checked };
//! person_sql.update_to(&jane, &update_to_jane).unwrap();
//!
//! let filter = Filter::NameEqual("Jane".to_string());
//! let updated_jane: Person = person_sql.select(filter.into()).unwrap().into_iter().nth(0).unwrap();
//! assert!(updated_jane.age == 28);
//!
//! // Check Jane's age
//! let p: Person = person_sql.select(Filter::AgeGreaterThan(27).into()).unwrap().into_iter().nth(0).unwrap();
//! assert!(p.age == 28);
//!
//! // Delete Jo
//! let filter = Filter::And(Box::new(Filter::NameEqual("Jo".to_string())), Box::new(Filter::AgeEqual(24)));
//! let jo: Person = person_sql.select(filter.into()).unwrap()
//!             .into_iter().nth(0).unwrap();
//! person_sql.delete(jo.into()).unwrap();
//!
//! let filter = Filter::NameEqual("Jo".to_string());
//! let jo: Vec<Person> = person_sql.select(filter.into()).unwrap();
//! assert!(jo.len() == 0);
//!
//! // Check that database only contains Jane
//! let persons: Vec<Person> = person_sql.select_all().unwrap();
//! assert!(persons.len() == 1);
//! assert!(persons[0].name.eq("Jane"));
//!
//! ```

mod sqltype;
mod implderive;
mod implfilter;
mod implselect;
mod implfilterwrapper;
mod utility;

use sqltype::SqlType;
use implderive::ImplDerive;

#[proc_macro_derive(DeriveSql)]
pub fn derive_sql(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let ast = syn::parse(input).unwrap();
  ImplDerive { ast: &ast }.generate().unwrap().into()
}

