//! Example demonstrating container and field attributes
//! Run with `cargo run --example attributes --features sqlite`
//!
use derive_sql::{Sqlable, SimpleFilter, SimpleLimit};

#[cfg(feature = "sqlite")]
#[derive(Debug)]
#[derive(derive_sql::DeriveSqlite)]
#[derive_sqlite(ident = PersonSql)]        // Use the nominated name for the SQLite wrapper structure (default: PersonSqlite)
#[derive_sqlite(table_name = "person_v1")] // Use the nominated name for the SQLite table (default: person)
struct Person {
  #[derive_sqlite(is_primary_key = true)] // Nominate the primary key (duplicates are not allowed)
  name: String,
  #[derive_sqlite(on_insert = set_age_on_insert)] // Nomimate a function to be run when inserting
  #[derive_sqlite(on_update = set_age_on_update)] // Nomimate a function to be run when updating
  age: u32,
  active: bool,
}

fn set_age_on_insert() -> u32 { 33 }
fn set_age_on_update() -> u32 { 26 }

#[cfg(feature = "sqlite")]
fn main() {
  let conn = rusqlite::Connection::open_in_memory().unwrap();
  if let Err(e) = sample(conn) {
    println!("simple::attributes failed with error: {}", e);
    std::process::exit(1);
  }
}

#[cfg(not(feature = "sqlite"))]
fn main() {
  println!("Feature `sqlite` required. Please run example using command `cargo run --example attributes --feature sqlite`");
}

#[cfg(feature = "sqlite")]
fn sample(conn: rusqlite::Connection) -> Result<(), Box<dyn std::error::Error>> {
  let mut db: PersonSql = conn.into();

  // Create Table in SQLite database
  println!("Create table");
  db.create_table()?;

  // Insert person into database
  println!("Insert person...");
  let person = Person { name: "Jo".to_string(), age: 44, active: true };
  let person = db.insert(person)?;
  assert!(person.name.eq("Jo"));
  println!("Insert person... ok");

  // Inserting another person with the same name (ie primary key) should fail
  println!("Insert person with duplicated name [primary key] fails...");
  assert!(db.insert(Person { name: "Jo".to_string(), age: 32, active: true }).is_err());
  println!("Insert person with duplicated name [primary key] fails... ok");

  // Check the run of on_insert function
  println!("Age is assigned when inserting...");
  let _ = db.insert(Person { name: "Jack".to_string(), age: 20, active: true})?;
  let persons: Vec<Person> = db.select(Box::new(SimpleFilter::try_from(("name", "Jack"))?.and(Box::new(SimpleLimit::try_from(1)?))))?;
  assert!(persons[0].age == 33);
  println!("Age is assigned when inserting... ok");

  // Check the 
  println!("Age is assigned when updating...");
  db.update(Box::new(SimpleFilter::try_from(("name", "Jack"))?.and(Box::new(SimpleLimit::try_from(1)?))), Person { name: "Jack".to_string(), age: 44, active: true })?;
  let persons: Vec<Person> = db.select(Box::new(SimpleFilter::try_from(("name", "Jack"))?.and(Box::new(SimpleLimit::try_from(1)?))))?;
  assert!(persons[0].age == 26);
  println!("Age is assigned when updating... ok");

  // Delete the table
  println!("Delete table");
  db.delete_table()?;

  println!("Example `attributes` ran successfully");
  Ok(())
}

