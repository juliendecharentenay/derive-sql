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
//! pub struct Person {
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
//! # pub struct Person {
//! #   name: String,
//! #   age: u32,
//! #   checked: bool,
//! # }
//! #
//! let conn = rusqlite::Connection::open_in_memory().unwrap();
//!
//! let person_sql = PersonSql::from_rusqlite(&conn).unwrap();
//!
//! // Create Table in SQL database
//! person_sql.create_table().unwrap();
//!
//! // Insert person into SQL database
//! let person = Person { name: "Jo".to_string(), age: 24, checked: false };
//! let _person = person_sql.insert(person).unwrap();
//!
//! // Retrieve list of persons from SQL database
//! assert!(person_sql.count_all().unwrap() == 1);
//! let persons: Vec<Person> = person_sql.select_all().unwrap();
//! assert!(persons.len() == 1);
//! assert!(persons[0].name.eq("Jo"));
//!
//! // Insert Jane
//! let jane = Person { name: "Jane".to_string(), age: 27, checked: false };
//! let _jane = person_sql.insert(jane).unwrap();
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
//!
//! ## Primary key 
//! The attribute `primary_key` can be added to a specific field to nominate that it is a unique
//! key identifying a specific record. This key will be used when querying, and updating records.
//! 
//! ```rust
//! # use derive_sql::DeriveSql;
//! #[derive(DeriveSql)]
//! pub struct ToDo {
//!   #[derive_sql(primary_key)]
//!   key: String,
//!   description: String,
//!   details: String,
//! }
//!
//! let conn = rusqlite::Connection::open_in_memory().unwrap();
//! let db = ToDoSql::from_rusqlite(&conn).unwrap();
//! db.create_table().unwrap();
//! let uid = nanoid::nanoid!();
//! let _todo = db.insert(ToDo {
//!   key: uid.clone(),
//!   description: "Call".to_string(),
//!   details: "a friend".to_string(),
//! }).unwrap();
//!
//! assert!(db.select_one(Filter::KeyEqual(uid.clone()).into()).unwrap().unwrap().details.eq("a friend"));
//!
//! // Update using primary key only
//! let r = db.update(uid.clone(), 
//!                 ToDo { 
//!                   key: nanoid::nanoid!(), 
//!                   description: "Call".to_string(), 
//!                   details: "John".to_string()
//!                 }).unwrap();
//! assert!(r.key.eq(uid.as_str()));
//!
//! assert!(db.select_one(Filter::KeyEqual(uid).into()).unwrap().unwrap().details.eq("John"));
//! 
//! ```
//! 
//! 
//! ## Auto-update field
//!
#![cfg_attr(feature = "chrono", doc = "```rust")]
#![cfg_attr(not(feature = "chrono"), doc = "```ignore")]
//! # use derive_sql::DeriveSql;
//! #[derive(DeriveSql)]
//! # #[cfg(feature = "chrono")]
//! pub struct ToDo {
//!   #[derive_sql(primary_key)]
//!   pk: usize,
//!   #[derive_sql(on_insert = "make_unique_key")]
//!   key: String,
//!   description: String,
//!   details: String,
//!   #[derive_sql(on_insert_update = "get_current_time")]
//!   timestamp: chrono::DateTime<chrono::Local>,
//! }
//!
//! fn make_unique_key() -> Result<String, Box<dyn std::error::Error>> {
//!   Ok(nanoid::nanoid!())
//! }
//!
//! # #[cfg(feature = "chrono")]
//! fn get_current_time() -> Result<chrono::DateTime<chrono::Local>, Box<dyn std::error::Error>> {
//!   Ok(chrono::Local::now())
//! }
//! 
//! let conn = rusqlite::Connection::open_in_memory().unwrap();
//! let db = ToDoSql::from_rusqlite(&conn).unwrap();
//! db.create_table().unwrap();
//!
//! let todo = db.insert(ToDo {
//!   pk: 0,
//!   key: "will_be_overwritten".to_string(),
//!   description: "a description".to_string(),
//!   details: "some details".to_string(),
//!   timestamp: chrono::Local::now().checked_sub_days(chrono::Days::new(10)).unwrap(), // An old date
//! }).unwrap();
//!
//! assert!(todo.key.eq("will_be_overwritten") == false);
//! println!("Duration: {:#?}", chrono::Local::now().signed_duration_since(todo.timestamp.clone()));
//! assert!(chrono::Local::now().signed_duration_since(todo.timestamp).num_seconds() < 60);
//!
//! let todo = db.update(0,
//!         ToDo {
//!           pk: 0,
//!           key: "new_key".to_string(),
//!           description: "a description".to_string(),
//!           details: "some more details".to_string(),
//!           timestamp: chrono::Local::now().checked_sub_days(chrono::Days::new(10)).unwrap(),
//!         }).unwrap();
//!
//! let todo = db.select_one(Filter::PkEqual(0).into()).unwrap().unwrap();
//! // Key field is not automatically updated
//! assert!(todo.key.eq("new_key"));
//! // timestamp field is automatically updated
//! println!("Duration: {:#?}", chrono::Local::now().signed_duration_since(todo.timestamp.clone()));
//! assert!(chrono::Local::now().signed_duration_since(todo.timestamp).num_seconds() < 60);
//!
//! ```
//! 
//! ## Nominated database constructor
//!
//! ```rust
//! # use derive_sql::DeriveSql;
//! #[derive(DeriveSql)]
//! #[derive_sql(rusqlite_connection = "make_connection")]
//! pub struct Entry {
//!   key: String,
//!   field: String, // TODO: struct with 1 field fails...
//! }
//!
//! fn make_connection() -> Result<rusqlite::Connection, Box<dyn std::error::Error>> {
//!   Ok(rusqlite::Connection::open_in_memory()?)
//! }
//! ```
//!
//!
//! ## Date & Time:
//! DateTime is supported using the `chrono` crate. Add the feature `chrono` to this crate to active and
//! remember to active the `chrono` feature on `rusqlite` - otherwise, expect compilation error.
//! Using the optional `chrono` feature:
//!
//! ```rust
//! # use derive_sql::DeriveSql;
//! # #[cfg(feature = "chrono")]
//! #[derive(DeriveSql)]
//! pub struct Meeting {
//!   start: chrono::DateTime<chrono::Local>,
//!   subject: String,
//! }
//! ```
//! Note: when running test, the above document test is not activated unless using the command
//! `cargo test --features chrono`.
//!

mod sqltype;
mod implderive;
mod implfilter;
mod implselect;
mod implfilterwrapper;
mod utility;

use sqltype::SqlType;
use implderive::ImplDerive;

#[proc_macro_derive(DeriveSql, attributes(derive_sql))]
pub fn derive_sql(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let ast = syn::parse(input).unwrap();
  ImplDerive { ast: &ast }.generate().unwrap().into()
}

