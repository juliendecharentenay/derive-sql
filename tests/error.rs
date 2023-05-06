use derive_sql::DeriveSql;

#[derive(DeriveSql)]
pub struct PrimaryKeyTest {
  #[derive_sql(primary_key)]
  key: String,
  #[derive_sql(on_insert = "function")]
  id: String,
}

pub fn function() -> Result<String, Box<dyn std::error::Error>> {
    Ok("id_inserted".to_string())
}

#[test]
fn main() {
  let conn = rusqlite::Connection::open_in_memory().unwrap();
  let db   = PrimaryKeyTestSql::from_rusqlite(&conn).unwrap();

  db.create_table().unwrap();

  // Check that the item has been modified on insertion
  let r    = db.insert(PrimaryKeyTest { key: "1".to_string(), id: "2".to_string() }).unwrap();
  assert!(r.id.eq("id_inserted"));

  // Check that the item stored is the modified one
  let r    = db.select_one(Filter::KeyEqual("1".to_string()).into()).unwrap().unwrap();
  assert!(r.id.eq("id_inserted"));
}

