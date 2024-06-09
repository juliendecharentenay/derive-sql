mod table; pub use table::artist;
mod query;
mod error; pub use error::{Result, MyError};

type Conn = derive_sql::sqlite::Conn;

struct AppState {
  conn: std::sync::Mutex<String>,
}

impl std::convert::TryFrom<&AppState> for artist::ArtistSqlite<derive_sql::sqlite::Conn> {
  type Error = MyError;
  fn try_from(v: &AppState) -> Result<artist::ArtistSqlite<derive_sql::sqlite::Conn>> {
    let mut db: artist::ArtistSqlite<_> = v.try_into()
    .map(|c: derive_sql::sqlite::Conn| c.into())?;
    let _ = db.create_table()?;
    Ok(db)
  }
}

impl std::convert::TryFrom<&AppState> for derive_sql::sqlite::Conn {
  type Error = MyError;
  fn try_from(v: &AppState) -> Result<derive_sql::sqlite::Conn> {
    let path = v.conn.lock().unwrap();
    Ok(rusqlite::Connection::open(path.as_str())?.into())
  }
}

/*
impl<'a> std::convert::TryFrom<&'a AppState> for &'a derive_sql::sqlite::Conn {
  type Error = MyError;
  fn try_from(v: &'a AppState) -> Result<&'a derive_sql::sqlite::Conn> {
    match &*v.conn.lock().unwrap() {
      Conn::Unset => Err(MyError::ConnectionNotInitialized),
      Conn::Sqlite((_, conn)) => Ok(conn),
    }
  }
}
*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env_logger::init();
  let state = actix_web::web::Data::new(AppState {
    conn: std::sync::Mutex::new("test.db3".to_string()),
  });

  actix_web::HttpServer::new(move || {
    actix_web::App::new()
      .app_data(state.clone())
      .route("/api/load",              actix_web::web::get().to(load))
      .route("/api/test",              actix_web::web::get().to(get_test))
      .route("/api/query/nationality", actix_web::web::get().to(get_artist_nationality))
  })
  .bind(("127.0.0.1", 8010))?
  .run()
  .await
}

async fn get_test(state: actix_web::web::Data<AppState>) -> actix_web::Result<impl actix_web::Responder>
{
  log::info!("Get test");
  let mut db: Conn = state.get_ref().try_into().map_err(actix_web::error::ErrorInternalServerError)?;
  create_table(&mut db).map_err(actix_web::error::ErrorInternalServerError)
  .map(|_| actix_web::HttpResponse::Ok())
}

fn create_table(db: &mut Conn) -> Result<()>
{
  use derive_sql::traits::CreateTable;
  Ok(db.create_table( &query::create_table::Statement::default() )?)
}


async fn get_artist_nationality(state: actix_web::web::Data<AppState>
  ) -> actix_web::Result<impl actix_web::Responder>
{
  log::info!("Get Artists nationalities");
  let db: Conn = state.get_ref().try_into().map_err(actix_web::error::ErrorInternalServerError)?;
  get_artist_nationality_handle(&db).await.map_err(actix_web::error::ErrorInternalServerError)
  .map(|r| actix_web::HttpResponse::Ok().json(r))
}

async fn get_artist_nationality_handle(db: &Conn) -> Result<Vec<query::nationality::Item>>
{
  use derive_sql::traits::Select;
  Ok(db.select(
    &derive_sql::generics::paginate::Paginate::from(query::nationality::Statement::default())
     .with_limit(10)
     .with_offset(5)
  )?)
}

async fn load(state: actix_web::web::Data<AppState>
  ) -> actix_web::Result<impl actix_web::Responder> 
{
  let mut db: artist::ArtistSqlite<_> = state.get_ref().try_into()
    .map_err(actix_web::error::ErrorInternalServerError)?;
  log::info!("Create table");
  db.create_table().map_err(actix_web::error::ErrorInternalServerError)?;
  load_handle(db).await.map_err(actix_web::error::ErrorInternalServerError)?;
  Ok(actix_web::HttpResponse::Ok().body(""))
}

async fn load_handle<T>(mut db: T) -> Result<()>
where T: derive_sql::Sqlable<Item = artist::Artist, Error = Box<dyn std::error::Error>, Selector = Box<dyn derive_sql::Selectable>>
{
  log::info!("Load");
  for artist in reqwest::get("https://github.com/MuseumofModernArt/collection/raw/main/Artists.json").await?
    .json::<Vec<artist::Artist>>().await?
    .into_iter() 
  {
    let _ = db.insert(artist)?;
    let count = db.count(Box::new(derive_sql::SimpleFilter::try_from(())?))?;
    if count % 100 == 0 { log::info!("{count} artists added"); }
  }

  log::info!("Complete");
  Ok(())
}
