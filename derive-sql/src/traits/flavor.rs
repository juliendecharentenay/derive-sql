use super::*;

mod sql_type; use sql_type::{SqlType, RawType};

/// Enum to advise on the SQL flavor supported by the connection.
pub enum Flavor {
  /// SQLite type connection
  SQLite,
  /// MySQL type connection
  MySQL,
  /// PostgreSQL type connection
  PostgreSQL,
  // Not any of the other...
  //Other,
}

impl std::fmt::Display for Flavor {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Flavor::SQLite => write!(f, "SQLite"),
      Flavor::MySQL => write!(f, "MySQL"),
      Flavor::PostgreSQL => write!(f, "PostgreSQL"),
    }
  }
}

impl Flavor {
  /// Retrieve row identifier. Supported for SQLite and PostgreSQL. 
  /// Returns an error for MySQL.
  pub fn row_id(&self) -> Result<String> {
    match self {
      Flavor::SQLite => Ok("rowid"),
      Flavor::MySQL => Err(Error::MySQLRowIdNotSupported),
      Flavor::PostgreSQL => Ok("CTID"),
    }.map(|s| s.to_string())
  }

  /// Handle column name
  pub fn column(&self, column: &str) -> Result<String> {
    match self {
      Flavor::SQLite
      | Flavor::MySQL => Ok(format!("`{column}`")),
      Flavor::PostgreSQL => Ok(format!("{column}")),
    }
  }

  /// Handle table name
  pub fn table(&self, table: &str) -> Result<String> {
    match self {
      Flavor::SQLite
      | Flavor::MySQL => Ok(format!("`{table}`")),
      Flavor::PostgreSQL => Ok(format!("{table}")),
    }
  }

  /// Handle values
  pub fn value(&self, i: usize) -> Result<String> {
    match self {
      Flavor::SQLite
      | Flavor::MySQL => Ok(format!("?")),
      Flavor::PostgreSQL => Ok(format!("${index}", index = i+1)),
    }
  }

  /// Conver to SQL type
  pub fn sql_type(&self, ty: &str) -> Result<SqlType> {
    /* match nullable {
      true  => Ok(SqlType::Nullable(self.raw_type(ty)?)),
      false => Ok(SqlType::NonNullable(self.raw_type(ty)?)),
    }*/
    Ok(SqlType::Nullable(self.raw_type(ty)?))
  }

  /// Convert to SQL raw type
  fn raw_type(&self, ty: &str) -> Result<RawType> {
    match (self, ty) {
        (Flavor::SQLite,     "i8") 
      | (Flavor::MySQL,      "i8") 
      | (Flavor::SQLite,     "u8") 
      | (Flavor::MySQL,      "u8") 
      | (Flavor::SQLite,     "i16") 
      | (Flavor::MySQL,      "i16") 
      | (Flavor::SQLite,     "u16") 
      | (Flavor::MySQL,      "u16") 
      | (Flavor::SQLite,     "i32") 
      | (Flavor::MySQL,      "i32") 
      | (Flavor::SQLite,     "u32") 
      | (Flavor::MySQL,      "u32") 
      | (Flavor::SQLite,     "i64") 
      | (Flavor::MySQL,      "i64") 
      | (Flavor::SQLite,     "u64") 
      | (Flavor::MySQL,      "u64") 
      | (Flavor::SQLite,     "usize") 
      | (Flavor::MySQL,      "usize") 
      => Ok(RawType::Int),

        (Flavor::PostgreSQL, "i8") 
      | (Flavor::PostgreSQL, "u8") 
      | (Flavor::PostgreSQL, "i16") 
      | (Flavor::PostgreSQL, "u16") 
      => Ok(RawType::SmallInt),

        (Flavor::PostgreSQL, "i32") 
      | (Flavor::PostgreSQL, "u32") 
      => Ok(RawType::Int),

        (Flavor::PostgreSQL, "i64") 
      | (Flavor::PostgreSQL, "u64") 
      | (Flavor::PostgreSQL, "usize") 
      => Ok(RawType::BigInt),

        (Flavor::SQLite,     "f32") 
      | (Flavor::MySQL,      "f32") 
      | (Flavor::SQLite,     "f64") 
      | (Flavor::MySQL,      "f64") 
      => Ok(RawType::Double),

        (Flavor::PostgreSQL, "f32") 
      => Ok(RawType::Real),

        (Flavor::PostgreSQL, "f64") 
      => Ok(RawType::Double),

        (Flavor::SQLite,     "bool") 
      | (Flavor::MySQL,      "bool") 
      | (Flavor::PostgreSQL, "bool") 
      => Ok(RawType::Boolean),

        (Flavor::SQLite,     "String") 
      | (Flavor::MySQL,      "String") 
      | (Flavor::PostgreSQL, "String") 
      => Ok(RawType::Text),

        (Flavor::SQLite,     "DateTime") 
      | (Flavor::MySQL,      "DateTime") 
      | (Flavor::PostgreSQL, "DateTime") 
      | (Flavor::SQLite,     "NaiveDateTime") 
      | (Flavor::MySQL,      "NaiveDateTime") 
      | (Flavor::PostgreSQL, "NaiveDateTime") 
      => Ok(RawType::DateTime),

        (Flavor::SQLite,     "NaiveDate") 
      | (Flavor::MySQL,      "NaiveDate") 
      | (Flavor::PostgreSQL, "NaiveDate") 
      => Ok(RawType::Date),
      
      _ => Err(Error::SqlTypeNotSupported(self.to_string(), ty.to_string())),

    }
  }
}
