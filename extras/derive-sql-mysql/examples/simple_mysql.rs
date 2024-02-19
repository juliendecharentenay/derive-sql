//! Simple example
//! Run with `cargo run --example simple --features mysql`
//!
use derive_sql::{Sqlable, SimpleFilter, SimpleLimit, SimpleOffset};

fn init_logger() {
  let _ = env_logger::builder()
  .filter_level(log::LevelFilter::max())
  .try_init();
}

#[derive(Debug)]
#[derive(derive_sql_mysql::DeriveMysql)]
struct Person {
  name: String,
  age: u32,
  active: bool,
}

fn main() {
  init_logger();
  let conn = mysql::Conn::new(
    mysql::Opts::from_url("mysql://test@localhost/simpledb").unwrap()
  ).unwrap();
  if let Err(e) = sample(conn) {
    log::error!("simple::sample failed with error: {}", e);
    std::process::exit(1);
  }
}

fn sample(conn: mysql::Conn) -> Result<(), Box<dyn std::error::Error>> {
  let mut db: PersonMysql<_> = derive_sql::mysql::Log::default().with(conn).into();

  // Create Table in SQLite database
  log::info!("Create table");
  db.create_table()?;

  // Insert person into database
  log::info!("Insert person...");
  let person = Person { name: "Jo".to_string(), age: 44, active: true };
  let person = db.insert(person)?;
  assert!(person.name.eq("Jo"));
  log::info!("Insert person... ok");

  let _ = db.insert(Person { name: "Jack".to_string(),  age: 44, active: true})?;
  let _ = db.insert(Person { name: "Harry".to_string(), age: 27, active: true})?;
  let _ = db.insert(Person { name: "Jack".to_string(),  age: 27, active: false})?;

  // Retrieve list of persons from SQL database
  log::info!("Retrieve list of persons...");
  let persons: Vec<Person> = db.select(Box::new(SimpleFilter::try_from(())?))?;
  assert!(persons.len() == 4);
  assert!(persons[0].name.eq("Jo"));
  assert!(persons[0].active == true);
  log::info!("Retrieve list of persons... ok");

  // Retrieve the list of persons with the name "Jack"
  log::info!("Retrieve list of persons with filter...");
  let persons: Vec<Person> = db.select(Box::new(SimpleFilter::try_from(("name", "Jack"))?))?;
  assert!(persons.len() == 2);
  log::info!("Retrieve list of persons with filter... ok");

  // Retrieve the first person with the name "Jack"
  log::info!("Retrieve list of persons with filter and limit...");
  let persons: Vec<Person> = db.select(Box::new(SimpleFilter::try_from(("name", "Jack"))?.and(Box::new(SimpleLimit::try_from(1)?))))?;
  assert!(persons.len() == 1);
  assert!(persons[0].age == 44);
  log::info!("Retrieve list of persons with filter and limit... ok");

  // Retrieve the second person (ie the first person after the first one) with the name "Jack"
  log::info!("Retrieve list of persons with filter, limit and offset...");
  let persons: Vec<Person> = db.select(Box::new(SimpleFilter::try_from(("name", "Jack"))?.and(Box::new(SimpleOffset::try_from((1,1))?))))?;
  assert!(persons.len() == 1);
  assert!(persons[0].age == 27);
  log::info!("Retrieve list of persons with filter, limit and offset... ok");

  // Delete all persons with the name "Jo"
  log::info!("Delete persons with filter...");
  db.delete(Box::new(SimpleFilter::try_from(("name", "Jo"))?))?;
  assert!(db.select(Box::new(SimpleFilter::try_from(())?))?.len() == 3);
  log::info!("Delete persons with filter... ok");

  // Update the first person with the name "Jack"...
  log::info!("Update persons with filter and limit...");
  db.update(Box::new(SimpleFilter::try_from(("name", "Jack"))?.and(Box::new(SimpleLimit::try_from(1)?))), Person { name: "Jo".to_string(), age: 44, active: true })?;
  assert!(db.select(Box::new(SimpleFilter::try_from(("name", "Jack"))?))?.len() == 1);
  log::info!("Update persons with filter and limit... ok");

  // Delete the table
  log::info!("Delete table");
  db.delete_table()?;

  log::info!("Example `simple` ran successfully");
  Ok(())
}

