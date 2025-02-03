use super::*;

pub enum SqlType {
  Nullable(RawType),
  NonNullable(RawType),
}

impl std::fmt::Display for SqlType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      SqlType::Nullable(v) => write!(f, "{v} NULL"),
      SqlType::NonNullable(v) => write!(f, "{v} NOT NULL"),
    }
  }
}

pub enum RawType {
  SmallInt,
  Int,
  BigInt,
  Boolean,
  Real,
  Double,
  Text,
  DateTime,
  Date,
}

impl std::fmt::Display for RawType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      RawType::SmallInt => write!(f, "SMALLINT"),
      RawType::Int      => write!(f, "INTEGER"),
      RawType::BigInt   => write!(f, "BIGINT"),
      RawType::Boolean  => write!(f, "BOOL"),
      RawType::Real     => write!(f, "FLOAT4"),
      RawType::Double   => write!(f, "FLOAT8"),
      RawType::Text     => write!(f, "TEXT"),
      RawType::DateTime => write!(f, "DATETIME"),
      RawType::Date     => write!(f, "DATE"),
    }
  }
}
