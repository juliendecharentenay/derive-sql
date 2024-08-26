use super::*;

struct Count { table: &'static str, }
impl derive_sql::traits::SelectStatement for Count {
  fn select_stmt(&self) -> derive_sql::Result<String> {
    Ok(format!("SELECT COUNT(*) FROM {table}",
      table = self.table,
    ))
  }
}

pub fn collect<C, R>(conn: &mut C) -> Result<()>
where C: derive_sql::traits::Connection<R>, R: derive_sql::traits::Row
{
  collect_impl(conn,
    || {
      tokio::runtime::Runtime::new()?.block_on( async {
        let artists = reqwest::get("https://github.com/MuseumofModernArt/collection/raw/main/Artists.json").await?
           .json::<Vec<table::artist::Artist>>().await?;
        Ok(artists)
      })
    },
    || {
      tokio::runtime::Runtime::new()?.block_on( async {
        let artworks = reqwest::get("https://github.com/MuseumofModernArt/collection/raw/main/Artworks.json").await?
         .json::<Vec<table::artworks::ArtworkRaw>>().await?;
        Ok(artworks)
      })
    },
  )
}

#[cfg(test)]
pub fn collect_test<C, R, F, G>(conn: &mut C, f: F, g: G) -> Result<()>
where C: derive_sql::traits::Connection<R>, R: derive_sql::traits::Row,
      F: FnOnce() -> Result<Vec<table::artist::Artist>>,
      G: FnOnce() -> Result<Vec<table::artworks::ArtworkRaw>>,
{
  collect_impl(conn, f, g)
}

fn collect_impl<C, R, F, G>(conn: &mut C, f: F, g: G) -> Result<()>
where C: derive_sql::traits::Connection<R>, R: derive_sql::traits::Row,
      F: FnOnce() -> Result<Vec<table::artist::Artist>>,
      G: FnOnce() -> Result<Vec<table::artworks::ArtworkRaw>>,
{
  use derive_sql::traits::{Table, InsertMultiple, SelectV2};

  { // Artists information
    log::info!("Collect artist information");
    let sql = table::artist::SqlArtist::default();
    sql.drop(conn)?; sql.create(conn)?;

    // Download artists
    log::info!("Download artists information");
    let artists: Vec<table::artist::Artist> = f()?;
    log::info!("Insert artists in database");
    sql.insert_multiple(conn, artists.iter())?;
    let count: Vec<u32> = Count { table: table::artist::SqlArtist::TABLE_NAME }.select(conn)?; let count = count[0];
    log::info!("{count} artists added");
  }

  { // Artwork
    log::info!("Collect artwork information");
    let artwork = table::artworks::SqlArtwork::default();
    artwork.drop(conn)?; artwork.create(conn)?;
    let attribution = table::artworks::SqlArtworkAttribution::default();
    attribution.drop(conn)?; attribution.create(conn)?;
    // Download
    log::info!("Downloading artworks");
    let artworks: Vec<table::artworks::ArtworkRaw> = g()?;

    log::info!("Insert artworks in database");
    artwork.insert_multiple(conn, artworks.iter().map(|a| a.as_artwork()).collect::<Vec<table::artworks::Artwork>>().iter())?;
    let count: Vec<u32> = Count { table: table::artworks::SqlArtwork::TABLE_NAME }.select(conn)?; let count = count[0];
    log::info!("{count} artworks added");

    log::info!("Insert artwork attributions in database");
    attribution.insert_multiple(conn, artworks.iter().map(|a| a.as_artwork_attribution()).flatten().collect::<Vec<table::artworks::ArtworkAttribution>>().iter())?;
    let count: Vec<u32> = Count { table: table::artworks::SqlArtworkAttribution::TABLE_NAME }.select(conn)?; let count = count[0];
    log::info!("{count} artwork attributions added");
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_collects_artists_and_artworks() -> Result<()> {
    let mut conn = rusqlite::Connection::open_in_memory()?;

    collect_impl(&mut conn,
      || Ok(vec![
        table::artist::Artist::make(1, "artist_1", "A. One", "A", 1, 2),
        table::artist::Artist::make(2, "artist_2", "A. Two", "A", 1, 2),
      ]),
      || Ok(vec![
        table::artworks::ArtworkRaw::make(10, "artwork 1", vec!["artist_1"], vec![1]),
        table::artworks::ArtworkRaw::make(11, "artwork 2", vec!["artist_1", "artist_2"], vec![1, 2]),
        table::artworks::ArtworkRaw::make(12, "artwork 3", vec!["artist_1"], vec![1]),
      ]),
    )?;

    // The number of artist stored should be 2
    let count: u32 = conn.query_row(
      format!("SELECT COUNT(*) FROM {table}", table=table::artist::SqlArtist::TABLE_NAME).as_str(),
      [],
      |row| row.get(0))?;
    assert!(count == 2);

    // The number of artwork stored should be 3
    let count: u32 = conn.query_row(
      format!("SELECT COUNT(*) FROM {table}", table=table::artworks::SqlArtwork::TABLE_NAME).as_str(),
      [],
      |row| row.get(0))?;
    assert!(count == 3);

    // The number of artwork attributions stored should be 4
    let count: u32 = conn.query_row(
      format!("SELECT COUNT(*) FROM {table}", table=table::artworks::SqlArtworkAttribution::TABLE_NAME).as_str(),
      [],
      |row| row.get(0))?;
    assert!(count == 4);

    Ok(())
  }
}
