use super::*;

#[derive(derive_sql::DeriveSqlStatement)]
struct QueryResult {
  nationality: Option<String>,
  count: usize,
}

#[derive(Default)]
struct SqlQuery {}
impl derive_sql::traits::SelectStatement for SqlQuery {
  fn select_stmt(&self) -> derive_sql::Result<String> {
    Ok(
      format!("SELECT {nationality},COUNT(*) AS count FROM {table} GROUP BY {nationality}",
        table=table::artist::SqlArtist::TABLE_NAME,
        nationality=table::artist::SqlArtist::NATIONALITY,
      )
    )
  }
}

pub fn query_nationalities<C, R>(conn: &mut C) -> Result<()>
where C: derive_sql::traits::Connection<R>, R: derive_sql::traits::Row,
{
  let results = query_impl(conn)?;
  println!("Nationality,Number of artists");
  for r in results.iter() {
    println!("{nationality},{count}",
      nationality=r.nationality.as_ref().map(|e| e.as_str()).unwrap_or("Unknown"),
      count=r.count,
    );
  }
  Ok(())
}

fn query_impl<C, R>(conn: &mut C) -> Result<Vec<QueryResult>>
where C: derive_sql::traits::Connection<R>, R: derive_sql::traits::Row,
{
  use derive_sql::traits::SelectV2;

  log::info!("Query top 25 nationality by count of artists");
  let results: Vec<QueryResult> = SqlQuery::default().select_with_filter_order_limit_offset(
    conn,
    &derive_sql::structs::filter::None::default(),           // No filtering
    &derive_sql::structs::Field::from("count").descending(), // By descending count
    25,  // First 25 record
    0,   // 0 offset
  )?;

  Ok(results)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_queries_nationalities() -> Result<()> {
    use derive_sql::traits::{Table, Insert};

    let mut conn = rusqlite::Connection::open_in_memory()?;

    let sql_artist = table::artist::SqlArtist::default();
    // Create table
    sql_artist.create(&mut conn)?;
    // Populate dummy data
    sql_artist.insert(&mut conn, &table::artist::Artist::make(1, "artist_1", "A. One",   "Nationality 1", 1, 2))?;
    sql_artist.insert(&mut conn, &table::artist::Artist::make(2, "artist_2", "A. Two",   "Nationality 2", 1, 2))?;
    sql_artist.insert(&mut conn, &table::artist::Artist::make(3, "artist_3", "A. Three", "Nationality 3", 1, 2))?;
    sql_artist.insert(&mut conn, &table::artist::Artist::make(4, "artist_4", "A. Four",  "Nationality 1", 1, 2))?;
    sql_artist.insert(&mut conn, &table::artist::Artist::make(5, "artist_5", "A. Five",  "Nationality 2", 1, 2))?;
    sql_artist.insert(&mut conn, &table::artist::Artist::make(6, "artist_6", "A. Six",   "Nationality 1", 1, 2))?;

    // Run query
    let results: Vec<QueryResult> = query_impl(&mut conn)?;

    // Check results
    assert!(results.len() == 3);
    assert!(results[0].nationality.as_ref().unwrap().eq("Nationality 1"));
    assert!(results[0].count == 3);
    assert!(results[1].nationality.as_ref().unwrap().eq("Nationality 2"));
    assert!(results[1].count == 2);
    assert!(results[2].nationality.as_ref().unwrap().eq("Nationality 3"));
    assert!(results[2].count == 1);

    Ok(())
  }
}
