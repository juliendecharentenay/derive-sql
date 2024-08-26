use super::*;

pub enum Row {
  Sqlite(derive_sql::proxy::sqlite::Row),
  MySql(derive_sql::proxy::mysql::Row),
}

impl derive_sql::traits::Row for Row {
  fn get_value(&self, i: usize) -> Option<derive_sql::Result<derive_sql::traits::Value>> {
    match self {
      Row::Sqlite(row) => row.get_value(i),
      Row::MySql(row) => row.get_value(i),
    }
  }
}

pub enum Connection {
  Sqlite(rusqlite::Connection),
  MySqlConn(mysql::Conn),
}

impl derive_sql::traits::Connection<Row> for Connection {
  fn flavor(&self) -> derive_sql::traits::Flavor {
    match self {
      Connection::Sqlite(conn) => conn.flavor(),
      Connection::MySqlConn(conn) => {
        conn.flavor()
      },
    }
  }

  fn execute_with_params<S, P>(&mut self, query: S, params: &P) -> derive_sql::Result<()>
  where S: std::convert::AsRef<str>,
        P: derive_sql::traits::Params
  {
    match self {
      Connection::Sqlite(conn) => conn.execute_with_params(query, params),
      Connection::MySqlConn(conn) => conn.execute_with_params(query, params),
    }
  }

  fn execute_with_params_iterator<'a, S, I, P>(&mut self, query: S, params_iter: I) -> derive_sql::Result<()>
  where S: std::convert::AsRef<str>,
        P: derive_sql::traits::Params + 'a,
        I: core::iter::IntoIterator<Item = &'a P>,
  {
    match self {
      Connection::Sqlite(conn) => conn.execute_with_params_iterator(query, params_iter),
      Connection::MySqlConn(conn) => conn.execute_with_params_iterator(query, params_iter),
    }
  }

  fn query<S>(&mut self, query: S) -> derive_sql::Result<Vec<Row>>
  where S: std::convert::AsRef<str>
  {
    Ok(
    match self {
      Connection::Sqlite(conn) => {
        conn.query(query)?.into_iter()
        .map(|r| Row::Sqlite(r))
        .collect()
      },
      Connection::MySqlConn(conn) => {
        conn.query(query)?.into_iter()
        .map(|r| Row::MySql(r))
        .collect()
      },
    }
    )
  }
}

pub fn make(config: &config::Database) -> Result<Connection> {
  match config {
    config::Database::Sqlite { filename } => Ok(Connection::Sqlite(rusqlite::Connection::open(filename)?) ),
    config::Database::MySql { hostname, username, password, database_name }
    => {
      let url = match password {
        Some(password) => format!("mysql://{username}:{password}@{hostname}/{database_name}"),
        None => format!("mysql://{username}@{hostname}/{database_name}"),
      };
      Ok(Connection::MySqlConn( mysql::Conn::new( mysql::Opts::from_url(url.as_str())?)? ))
    },
  }
}
