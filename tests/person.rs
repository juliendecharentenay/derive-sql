use std::error::Error;
use derive_sql::DeriveSql;

#[derive(DeriveSql)]
struct Person {
  name: String,
  age:  u32,
}

#[test]
fn main() {
  let conn = rusqlite::Connection::open_in_memory().unwrap();
  if let Err(e) = sample(&conn) {
    println!("person::sample failed with error: {}", e);
    std::process::exit(1);
  }
}

pub fn sample(conn: &rusqlite::Connection) -> Result<(), Box<dyn Error>> {
  // Create Table in SQL database
  Person::create_table(&conn)?;

  // Insert person into SQL database
  let person = Person { name: "Jo".to_string(), age: 44 };
  person.insert(&conn)?;

  // Retrieve list of persons from SQL database
  let persons: Vec<Person> = Person::select(&conn)?;
  assert!(persons.len() == 1);
  assert!(persons[0].name.eq("Jo"));

  Ok(())
}

