//! Example demonstrating container and field attributes
//! Run with `cargo run --example attributes --features mysql`
//!
use derive_sql::{Sqlable, SimpleFilter, SimpleLimit};

#[derive(Debug)]
#[derive(derive_sql_mysql::DeriveMysql)]
#[derive_sqlite(ident = PersonSql)]        // Use the nominated name for the MySQL wrapper structure (default: PersonMysql)
#[derive_sqlite(table_name = "person_v1")] // Use the nominated name for the MySQL table (default: person)
struct Person {
  #[derive_sqlite(is_primary_key = true)] // Nominate the primary key (duplicates are not allowed)
  key: u32,
  name: String,
  #[derive_sqlite(on_insert = set_age_on_insert)] // Nomimate a function to be run when inserting
  #[derive_sqlite(on_update = set_age_on_update)] // Nomimate a function to be run when updating
  age: u32,
  active: bool,
}

fn set_age_on_insert() -> u32 { 33 }
fn set_age_on_update() -> u32 { 26 }

fn main() {
  let conn = mysql::Conn::new(
    mysql::Opts::from_url("mysql://test@localhost/simpledb").unwrap()
  ).unwrap();
  if let Err(e) = sample(conn) {
    println!("simple::attributes failed with error: {}", e);
    std::process::exit(1);
  }
}

fn sample(conn: mysql::Conn) -> Result<(), Box<dyn std::error::Error>> {
  let mut db: PersonSql<_> = conn.into();

  // Create Table in MySQL database
  println!("Create table");
  db.create_table()?;

  // Insert person into database
  println!("Insert person...");
  let person = Person { key: 1, name: "Jo".to_string(), age: 44, active: true };
  let person = db.insert(person)?;
  assert!(person.name.eq("Jo"));
  println!("Insert person... ok");

  // Inserting another person with the same name (ie primary key) should fail
  println!("Insert person with duplicated name [primary key] fails...");
  assert!(db.insert(Person { key: 1, name: "Jo".to_string(), age: 32, active: true }).is_err());
  println!("Insert person with duplicated name [primary key] fails... ok");

  // Check the run of on_insert function
  println!("Age is assigned when inserting...");
  let _ = db.insert(Person { key: 2, name: "Jack".to_string(), age: 20, active: true})?;
  let persons: Vec<Person> = db.select(Box::new(SimpleFilter::try_from(("name", "Jack"))?.and(Box::new(SimpleLimit::try_from(1)?))))?;
  assert!(persons[0].age == 33);
  println!("Age is assigned when inserting... ok");

  // Check the 
  println!("Age is assigned when updating...");
  db.update(Box::new(SimpleFilter::try_from(("name", "Jack"))?.and(Box::new(SimpleLimit::try_from(1)?))), Person { key: 3, name: "Jack".to_string(), age: 44, active: true })?;
  let persons: Vec<Person> = db.select(Box::new(SimpleFilter::try_from(("name", "Jack"))?.and(Box::new(SimpleLimit::try_from(1)?))))?;
  assert!(persons[0].age == 26);
  println!("Age is assigned when updating... ok");

  // Delete the table
  println!("Delete table");
  db.delete_table()?;

  println!("Example `attributes` ran successfully");
  Ok(())
}

