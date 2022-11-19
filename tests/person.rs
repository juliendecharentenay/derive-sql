use std::error::Error;
use derive_sql::DeriveSql;

#[derive(DeriveSql)]
pub struct Person {
  name: String,
  age:  u32,
  confirmed: bool,
}

#[test]
fn main() {
  let conn = rusqlite::Connection::open_in_memory().unwrap();
  if let Err(e) = sample(&conn) {
    println!("person::sample failed with error: {}", e);
    std::process::exit(1);
  }
}

fn sample(conn: &rusqlite::Connection) -> Result<(), Box<dyn Error>> {
  let db = PersonSql::from_rusqlite(conn)?;

  // Create Table in SQL database
  db.create_table()?;

  // Insert person into SQL database
  let person = Person { name: "Jo".to_string(), age: 44, confirmed: true };
  db.insert(&person)?;

  // Retrieve list of persons from SQL database
  let persons: Vec<Person> = db.select_all()?;
  assert!(persons.len() == 1);
  assert!(persons[0].name.eq("Jo"));
  assert!(persons[0].confirmed == true);

  Ok(())
}

