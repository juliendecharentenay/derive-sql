//! Example demonstrating container and field attributes
//! Run with `cargo run --example attributes --features mysql,sqlite`
//!
use derive_sql::{traits, Result, mysql};

fn init_logger() {
  let _ = env_logger::builder()
  .filter_level(log::LevelFilter::max())
  .try_init();
}

#[derive(Debug)]
#[derive(derive_sql_statement::DeriveSqlStatement)]
#[derive_sql(ident = PersonSql)]        // Use the nominated name for the MySQL wrapper structure (default: PersonMysql)
#[derive_sql(table_name = "person_v1")] // Use the nominated name for the MySQL table (default: person)
struct Person {
  #[derive_sqlite(is_primary_key = true)] // Nominate the primary key (duplicates are not allowed)
  key: u32,
  name: String,
  age: u32,
  active: bool,
}

fn main() {
  init_logger();
  log::info!("Run attributes with mysql connection");
  let mut conn = mysql::Conn::new(
    mysql::Opts::from_url("mysql://test@localhost/simpledb").unwrap()
  ).unwrap();
  if let Err(e) = sample(&mut conn) {
    println!("attributes failed with error: {}", e);
    std::process::exit(1);
  }
}

fn sample<C, R>(conn: &mut C) -> Result<()>
where C: traits::Connection<R>,
      R: traits::Row,
{
  use derive_sql::traits::{Table, SelectV2, Insert, Delete, Update};
  let db = PersonSql::default();

  // Drop table if exists
  db.drop(conn)?;

  // Create Table in MySQL database
  log::info!("Create table");
  db.create(conn)?;

  // Insert person into database
  log::info!("Insert person...");
  let person = Person { key: 1, name: "Jo".to_string(), age: 44, active: true };
  db.insert(conn, &person)?;
  log::info!("Check person insertion");
  let person: Vec<Person> = db.select(conn)?;
  assert!(person[0].name.eq("Jo"));
  log::info!("Insert person... ok");

  // Inserting another person with the same name (ie primary key) should fail
  log::info!("Insert person with duplicated name [primary key] fails...");
  assert!(db.insert(conn, &Person { key: 1, name: "Jo".to_string(), age: 32, active: true }).is_err());
  log::info!("Insert person with duplicated name [primary key] fails... ok");

  // Delete the table
  log::info!("Delete table");
  db.drop(conn)?;

  log::info!("Example `attributes` ran successfully");
  Ok(())
}

