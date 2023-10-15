//! Example showing the use of the provided `SqlableTest` struct. A struct implementing `Sqlable` that can be used for testing purposes.
//! Run with `cargo run --example test --features with-mock`
//!
#[cfg(feature = "with-mock")]
use derive_sql::Sqlable;

#[cfg(not(feature = "with-mock"))]
fn main() {
  println!("Feature `with-mock` required. Please run example using the command `cargo run --example test --features with-mock`");
}

#[cfg(feature = "with-mock")]
fn main() {
  if let Err(e) = sample() {
    eprintln!("test::sample failed with error: {e}");
    std::process::exit(1);
  }
}

#[derive(Default)]
struct Person {
}

#[cfg(feature = "with-mock")]
fn sample() -> Result<(), Box<dyn std::error::Error>> {
  println!("Creating MockSqlable");
  let mut db_test = derive_sql::MockSqlable::default()
    .with_on_insert(Box::new(|| Person { }))
    .with_on_select(Box::new(|| Vec::new()));
  println!("Running algorithm with MockSqlable");
  algorithm_to_test(&mut db_test)?;
  println!("Checking that calls took place");
  assert!(db_test.count_calls.borrow().len() == 0);
  assert!(db_test.insert_calls.borrow().len() == 1);
  assert!(db_test.select_calls.borrow().len() == 1);
  assert!(db_test.update_calls.borrow().len() == 0);
  assert!(db_test.delete_calls.borrow().len() == 0);
  println!("Test successfull");
  Ok(())
}

#[cfg(feature = "with-mock")]
fn algorithm_to_test<T>(db: &mut T) -> Result<(), Box<dyn std::error::Error>>
where T: Sqlable<Item = Person, Error = Box<dyn std::error::Error>, Selector = ()>,
{
  let _ = db.insert(Person { });
  let _ = db.select(())?;
  Ok(())
}

