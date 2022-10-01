use std::error::Error;
use derive_sql::DeriveSql;

#[derive(DeriveSql, Debug)]
pub struct Contact {
  name: String,
  phone_number: String,
  email: String,
}

#[test]
fn main() {
  let conn = rusqlite::Connection::open_in_memory().unwrap();
  if let Err(e) = sample(&conn) {
    println!("contact::sample failed with error: {}", e);
    std::process::exit(1);
  }
}

fn sample(conn: &rusqlite::Connection) -> Result<(), Box<dyn Error>> {
  let db = ContactSql::from_rusqlite(conn)?;
  db.create_table()?;

  // Insert a new contact
  let contact = Contact { name: "John Doe".to_string(), phone_number: "01223456789".to_string(), email: "john@doe.com".to_string() };
  db.insert(&contact)?;

  // Add another contact
  db.insert(
    &Contact { name: "Jane Doe".to_string(), phone_number: "00000000".to_string(), email: "jane@doe.com".to_string() }
  )?;

  // Lookup John Doe's contact
  let contact = db.select()?
        .into_iter()
        .find(|c| c.name.eq("John Doe"))
        .ok_or("Unable to find John Doe's contact")?;

  // Update contact
  let update = Contact { name: contact.name.clone(), phone_number: "987654321".to_string(), email: contact.email.clone() };
  db.update_to(&contact, &update)?;


  // List all contacts
  println!("List all contact stored in table");
  for contact in db.select()?.iter() {
    println!("{}: {} / {}", contact.name, contact.phone_number, contact.email);
  }

  Ok(())
}

#[test]
fn test_2_create_table_statements() -> Result<(), Box<dyn Error>> {
  let conn = rusqlite::Connection::open_in_memory()?;
  let db = ContactSql::from_rusqlite(&conn)?;
  assert!(db.table_exists()? == false);

  db.create_table()?;
  assert!(db.table_exists()? == true);

  db.create_table()?;
  assert!(db.table_exists()? == true);

  Ok(())
}
