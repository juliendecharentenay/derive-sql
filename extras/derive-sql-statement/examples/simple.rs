//! Simple example
//! Run with `cargo run --example simple --features mysql`
//!
use derive_sql::{traits, structs::Field, structs::filter, structs::order};

fn init_logger() {
  let _ = env_logger::builder()
  .filter_level(log::LevelFilter::max())
  .try_init();
}

#[derive(Debug)]
#[derive(derive_sql_statement::DeriveSqlStatement)]
struct Person {
  name: String,
  age: u32,
  active: bool,
  nickname: Option<String>,
}

fn main() {
  init_logger();
  log::info!("=================================");
  log::info!("Run simple with mysql connection");
  let mut conn = mysql::Conn::new(
    mysql::Opts::from_url("mysql://test@localhost/simpledb").unwrap()
  ).unwrap();
  if let Err(e) = sample(&mut conn) {
    log::error!("simple::sample failed to run with mysql with error: {}", e);
    std::process::exit(1);
  }

  log::info!("=================================");
  log::info!("Run simple with sqlite connection");
  let mut conn = rusqlite::Connection::open_in_memory().unwrap();
  if let Err(e) = sample(&mut conn) {
    log::error!("simple::sample failed to run with sqlite with error: {}", e);
    std::process::exit(1);
  }
}

fn sample<Conn, Row>(conn: &mut Conn) -> Result<(), Box<dyn std::error::Error>> 
where Conn: traits::Connection<Row>,
      Row: traits::Row,
{
  use derive_sql::traits::{Table, SelectV2, Insert, Delete, Update};
  /*
  let mut log = derive_sql::proxy::Log::from_connection_level(conn, log::Level::Info);
  let conn = &mut log;
  */

  let db = SqlPerson::default();

  // Drop table if it exists
  db.drop(conn)?;

  // Create Table in SQLite database
  log::info!("Create table");
  db.create(conn)?;

  // Insert person into database
  log::info!("Insert person...");
  let person = Person { name: "Jo".to_string(), age: 44, active: true, nickname: None, };
  db.insert(conn, &person)?;
  log::info!("Check person insertion...");
  let persons: Vec<Person> = db.select(conn)?;
  assert!(persons[0].name.eq("Jo"));
  log::info!("Insert person... ok");

  let _ = db.insert(conn, &Person { name: "Jack".to_string(),  age: 44, active: true, nickname: None,})?;
  let _ = db.insert(conn, &Person { name: "Harry".to_string(), age: 27, active: true, nickname: Some("The H".to_string()),})?;
  let _ = db.insert(conn, &Person { name: "Jack".to_string(),  age: 27, active: false, nickname: None,})?;

  // Retrieve list of persons from SQL database
  log::info!("Retrieve list of persons...");
  let persons: Vec<Person> = db.select(conn)?;
  assert!(persons.len() == 4);
  assert!(persons[0].name.eq("Jo"));
  assert!(persons[0].active == true);
  log::info!("Retrieve list of persons... ok");

  // Retrieve the list of persons with the name "Jack"
  log::info!("Retrieve list of persons with filter...");
  let persons: Vec<Person> = db.select_with_filter(conn, &Field::from("name").eq("Jack"))?;
  assert!(persons.len() == 2);
  log::info!("Retrieve list of persons with filter... ok");

  // Retrieve the first person with the name "Jack"
  log::info!("Retrieve list of persons with filter and limit...");
  let persons: Vec<Person> = db.select_with_filter_order_limit_offset(conn, &Field::from("name").eq("Jack"), &order::None::default(), 1, 0)?;
  assert!(persons.len() == 1);
  assert!(persons[0].age == 44);
  assert!(persons[0].nickname.is_none());
  log::info!("Retrieve list of persons with filter and limit... ok");

  // Retrieve the second person (ie the first person after the first one) with the name "Jack"
  log::info!("Retrieve list of persons with filter, limit and offset...");
  let persons: Vec<Person> = db.select_with_filter_order_limit_offset(conn, &Field::from("name").eq("Jack"),
    &order::None::default(), 1, 1)?;
  assert!(persons.len() == 1);
  assert!(persons[0].age == 27);
  log::info!("Retrieve list of persons with filter, limit and offset... ok");

  // Count the number of persons with a nickname
  log::info!("Retrieve list with filtering on null/not null");
  let persons: Vec<Person> = db.select_with_filter(conn, &Field::from("nickname").is_some())?;
  assert!(persons.len() == 1);
  assert!(persons[0].name.eq("Harry"));
  let persons: Vec<Person> = db.select_with_filter(conn, &Field::from("nickname").is_none())?;
  assert!(persons.len() == 3);

  // Delete all persons with the name "Jo"
  log::info!("Delete persons with filter...");
  db.delete_with_filter(conn, &Field::from("name").eq("Jo"))?;
  let persons: Vec<Person> = db.select(conn)?;
  assert!(persons.len() == 3);
  log::info!("Delete persons with filter... ok");

  // Update the first person with the name "Jack"...
  log::info!("Update persons with filter and limit...");
  db.update_with_filter_order_limit_offset(conn,
    &Field::from("name").eq("Jack"),
    &order::None::default(),
    1, // limit
    0, // offset
    &Person { name: "Jo".to_string(), age: 44, active: true, nickname: None, },
  )?;
  let persons: Vec<Person> = db.select_with_filter(conn, &Field::from("name").eq("Jack"))?;
  assert!(persons.len() == 1);
  log::info!("Update persons with filter and limit... ok");

  // Delete the table
  log::info!("Delete table");
  db.drop(conn)?;

  log::info!("Example `simple` ran successfully");
  Ok(())
}

