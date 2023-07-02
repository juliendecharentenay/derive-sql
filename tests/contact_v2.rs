use std::error::Error;
// use derive_sql::DeriveSql;

#[derive(derive_sql::DeriveSql2, Debug)]
#[derive_sql(rusqlite_connection = "make_rusqlite")]
pub struct Contact {
  name: String,
  phone_number: String,
  email: String,
}

fn make_rusqlite() -> Result<rusqlite::Connection, Box<dyn std::error::Error>> {
  Ok(rusqlite::Connection::open_in_memory()?)
}

#[test]
fn main() -> Result<(), Box<dyn Error>> {
  let conn = rusqlite::Connection::open_in_memory().unwrap();
  let db = ContactSql::from_rusqlite(&conn)?;
  db.create_table()?;

  // Insert a new contact
  let contact = Contact { name: "John Doe".to_string(), phone_number: "01223456789".to_string(), email: "john@doe.com".to_string() };
  let _contact = db.insert(contact)?;

  // Add another contact
  let _contact = db.insert(
    Contact { name: "Jane Doe".to_string(), phone_number: "00000000".to_string(), email: "jane@doe.com".to_string() }
  )?;

  // Lookup John Doe's contact
  let contact = db.select_all()?
        .into_iter()
        .find(|c| c.name.eq("John Doe"))
        .ok_or("Unable to find John Doe's contact")?;

  // Update contact
  let update = Contact { name: contact.name.clone(), phone_number: "987654321".to_string(), email: contact.email.clone() };
  let _update = db.update(sql::Filter::NameEqual("Jane Doe".to_string()).try_into()?, update)?;
  // assert!(db.count_all()? == 2);
  assert!(db.select_all()?.len() == 2);

  // List all contacts
  // println!("List all {} contacts stored in table", db.count_all()?);
  println!("List all contacts stored in table");
  for contact in db.select_all()?.iter() {
    println!("{}: {} / {}", contact.name, contact.phone_number, contact.email);
  }

/*
  // Check select with Filter::None and Filter::All
  assert!(db.select(Filter::None.into())?.len() == 0);
  assert!(db.select(Filter::All.into())?.len() == 2);

  // Empty all contacts
  db.delete_all()?;
  println!("Number of contact after deleting database: {}", db.count_all()?);
  assert!(db.count_all()? == 0);
  */

  Ok(())
}

#[test]
fn test_2_create_table_statements() -> Result<(), Box<dyn Error>> {
  let db = ContactSql::from_database()?;
  assert!(db.table_exists()? == false);

  db.create_table()?;
  assert!(db.table_exists()? == true);

  db.create_table()?;
  assert!(db.table_exists()? == true);

  db.delete_table()?;
  assert!(db.table_exists()? == false);

  Ok(())
}
