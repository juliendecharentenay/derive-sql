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
      format!(r#"
SELECT {nationality},COUNT(*) AS count FROM {table_artist}
INNER JOIN {table_attribution}
  ON {table_attribution}.{attribution_constituent_id}={table_artist}.{artist_constituent_id}
INNER JOIN {table_artwork}
  ON {table_artwork}.{artwork_object_id}={table_attribution}.{attribution_object_id}
  GROUP BY {nationality}
        "#,
        table_artist=table::artist::SqlArtist::TABLE_NAME,
        nationality=table::artist::SqlArtist::NATIONALITY,
        artist_constituent_id=table::artist::SqlArtist::CONSTITUENT_ID,
        table_attribution=table::artworks::SqlArtworkAttribution::TABLE_NAME,
        attribution_constituent_id=table::artworks::SqlArtworkAttribution::CONSTITUENT_ID,
        attribution_object_id=table::artworks::SqlArtworkAttribution::OBJECT_ID,
        table_artwork=table::artworks::SqlArtwork::TABLE_NAME,
        artwork_object_id=table::artworks::SqlArtwork::OBJECT_ID,
      )
    )
  }
}

pub fn query_artwork_nationalities<C, R>(conn: &mut C) -> Result<()>
where C: derive_sql::traits::Connection<R>, R: derive_sql::traits::Row,
{
  let results = query_impl(conn)?;
  println!("Nationality,Number of artwork");
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

  log::info!("Query top 25 nationality by count of artworks");
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
  fn it_queries() -> Result<()> {
    let mut conn = rusqlite::Connection::open_in_memory()?;

    // Populate database
    collect::collect_test(&mut conn,
      || Ok(vec![
        table::artist::Artist::make(1, "artist_1", "A. One", "Nationality A", 1, 2),
        table::artist::Artist::make(2, "artist_2", "A. Two", "Nationality B", 1, 2),
        table::artist::Artist::make(3, "artist_3", "A. Three", "Nationality C", 1, 2),
      ]),
      || Ok(vec![
        table::artworks::ArtworkRaw::make(10, "artwork 1", vec!["artist_1", "artist_2", "artist_3"], vec![1, 2, 3]),
        table::artworks::ArtworkRaw::make(11, "artwork 2", vec!["artist_2", "artist_3"], vec![2, 3]),
        table::artworks::ArtworkRaw::make(12, "artwork 3", vec!["artist_3"], vec![3]),
      ]),
    )?;

    // Run query and check results
    let results = query_impl(&mut conn)?;
    assert!(results.len() == 3);
    assert!(results[0].nationality.as_ref().unwrap().eq("Nationality C"));
    assert!(results[0].count == 3);
    assert!(results[1].nationality.as_ref().unwrap().eq("Nationality B"));
    assert!(results[1].count == 2);
    assert!(results[2].nationality.as_ref().unwrap().eq("Nationality A"));
    assert!(results[2].count == 1);

    Ok(())
  }
}
