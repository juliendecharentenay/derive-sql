mod table;
mod error; pub use error::{Result, Error};
mod config;
mod database;
mod run;

// Locally import library exported in `derive_sql`
use derive_sql::{rusqlite, mysql};

fn main() {
  env_logger::init();
  log::debug!("Main");
  if let Err(e) = config::parse()
  .and_then(|config| {
    log::debug!("Config: {config:?}");
    run(config)
  }) {
    log::error!("Error: {e}");
  }
}

fn run(config: config::Config) -> Result<()> {
  let mut db = database::make(&config.db)?;
  match config.action {
    config::Action::Collect => run::collect(&mut db),
    config::Action::QueryNationalities => run::query_nationalities(&mut db),
    config::Action::QueryArtworkNationalities => run::query_artwork_nationalities(&mut db),
  }
}

