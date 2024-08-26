use super::*;

#[derive(Debug)]
pub struct Config {
  pub db: Database,
  pub action: Action,
}

#[derive(Debug)]
pub enum Database {
  Sqlite { filename: String, },
  MySql  { hostname: String, username: String, password: Option<String>, database_name: String, },
}

#[derive(Debug)]
pub enum Action {
  Collect,
  QueryNationalities,
  QueryArtworkNationalities,
}

pub fn parse() -> Result<Config> {
  let matches = clap::Command::new("mona")
    .subcommand_required(true)
    .subcommand(
      clap::Command::new("collect")
      .about("Define the action to take")
    )
    .subcommand(
      clap::Command::new("query_nationalities")
      .about("List nationalities by decreasing number of artists")
    )
    .subcommand(
      clap::Command::new("query_artwork_nationalities")
      .about("List nationalities by decreasing number of artwork")
    )
    .arg(
      clap::Arg::new("database")
      .short('d').long("database")
      .value_parser(["sqlite", "mysql"])
      .default_value("sqlite")
    )
    .arg(
      clap::Arg::new("sqlite_filename")
      .short('f').long("filename")
      .help("Enter the SQLite filename")
    )
    .arg(
      clap::Arg::new("mysql_host")
      .short('m').long("hostname")
      .help("Enter the MySQL hostname")
    )
    .arg(
      clap::Arg::new("mysql_username")
      .short('u').long("username")
      .help("Enter the MySQL username")
    )
    .arg(
      clap::Arg::new("mysql_password")
      .short('p').long("password")
      .help("Enter the MySQL password")
    )
    .arg(
      clap::Arg::new("mysql_database_name")
      .short('n').long("database_name")
      .help("Enter the MySQL database name")
    )
    .get_matches();

  let db = match matches.get_one::<String>("database").map(|v| v.as_str()) {
    Some("sqlite") => {
      if let Some(filename) = matches.get_one::<String>("sqlite_filename") {
        Ok( Database::Sqlite { filename: filename.clone() } )
      } else {
        Err(Error::UnspecifiedSqliteFilename)
      }
    },
    Some("mysql") => {
      if let (Some(hostname), Some(username), password, Some(database_name)) 
        = (matches.get_one::<String>("mysql_host"),
           matches.get_one::<String>("mysql_username"),
           matches.get_one::<String>("mysql_password"),
           matches.get_one::<String>("mysql_database_name")) {
        Ok( Database::MySql { 
              hostname: hostname.clone(), 
              username: username.clone(), 
              password: password.cloned(),
              database_name: database_name.clone(),
            } )
      } else {
        Err(Error::UnspecifiedMysqlParameters)
      }
    },
    Some(v) => Err(Error::UnsupportedDatabaseType(v.to_string())),
    None => Err(Error::UnspecifiedDatabaseType),
  }?;

  let action = if matches.subcommand_matches("collect").is_some() {
    Ok( Action::Collect )
  } else if matches.subcommand_matches("query_nationalities").is_some() {
    Ok( Action::QueryNationalities )
  } else if matches.subcommand_matches("query_artwork_nationalities").is_some() {
    Ok( Action::QueryArtworkNationalities )
  } else {
    Err(Error::UnspecifiedAction)
  }?;
  
  Ok( Config { db, action } )
}

