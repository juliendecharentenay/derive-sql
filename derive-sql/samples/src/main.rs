use rusqlite;
use derive_sql::DeriveSql;

#[derive(DeriveSql)]
struct Person {
  name: String,
  age:  u32,
}

fn main() {
 let conn = rusqlite::Connection::open_in_memory().unwrap();

 // Create Table in SQL database
 Person::create_table(&conn).unwrap();

 // Insert person into SQL database
 let person = Person { name: "Jo".to_string(), age: 44 };
 person.insert(&conn).unwrap();

 // Retrieve list of persons from SQL database
 let persons: Vec<Person> = Person::select(&conn).unwrap();
 assert!(persons.len() == 1);
 assert!(persons[0].name.eq("Jo"));
}
