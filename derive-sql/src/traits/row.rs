use super::*;

#[derive(Debug, Clone)]
pub enum Value {
  Null,
  Integer(i64),
  UInteger(u64),
  Real(f64),
  Text(String),
  Blob(Vec<u8>),
#[cfg(feature = "mysql")]
  MysqlValue(::mysql::Value),
}


#[cfg(feature = "mysql")]
impl std::convert::TryFrom<::mysql::Value> for Value {
  type Error = Error;
  fn try_from(v: ::mysql::Value) -> Result<Self> {
    Ok(Value::MysqlValue(v))
  }
}

pub trait TryFromValue {
  fn try_from(v: Value) -> Result<Self> where Self: Sized;
}

impl TryFromValue for Vec<u8> {
  fn try_from(v: Value) -> Result<Self> { 
    match v {
      Value::Blob(v) => Ok(v),
#[cfg(feature = "mysql")]
      Value::MysqlValue(v) => Ok(::mysql::from_value_opt(v)?),
      _ => Err(Error::InvalidTypeFor("Vec<u8>".to_string())),
    } 
  }
}

impl TryFromValue for bool {
  fn try_from(v: Value) -> Result<Self> { 
    match v {
      Value::Integer(v) => Ok(v != 0),
#[cfg(feature = "mysql")]
      Value::MysqlValue(v) => Ok(::mysql::from_value_opt(v)?),
      _ => Err(Error::InvalidTypeFor("bool".to_string())),
    }
  }
} 

impl TryFromValue for u32 {
  fn try_from(v: Value) -> Result<Self> { 
    match v {
      Value::UInteger(v) => Ok(v.try_into()?),
      Value::Integer(v)  => Ok(v.try_into()?),
#[cfg(feature = "mysql")]
      Value::MysqlValue(v) => Ok(::mysql::from_value_opt(v)?),
      _ => Err(Error::InvalidTypeForFrom("u32".to_string(), format!("{v:?}"))),
    }
  }
} 

impl TryFromValue for i64 {
  fn try_from(v: Value) -> Result<Self> { 
    match v {
      Value::Integer(v) => Ok(v),
#[cfg(feature = "mysql")]
      Value::MysqlValue(v) => Ok(::mysql::from_value_opt(v)?),
      _ => Err(Error::InvalidTypeFor("i64".to_string())),
    }
  }
}

impl TryFromValue for f64 {
  fn try_from(v: Value) -> Result<Self> { 
    match v {
      Value::Real(v) => Ok(v),
#[cfg(feature = "mysql")]
      Value::MysqlValue(v) => Ok(::mysql::from_value_opt(v)?),
      _ => Err(Error::InvalidTypeFor("f64".to_string())),
    }
  }
}

impl TryFromValue for String {
  fn try_from(v: Value) -> Result<Self> { 
    match v {
      Value::Text(v) => Ok(v),
      Value::Blob(v) => Ok(String::from_utf8(v)?),
#[cfg(feature = "mysql")]
      Value::MysqlValue(v) => Ok(::mysql::from_value_opt(v)?),
      _ => Err(Error::InvalidTypeForFrom("String".to_string(), format!("{v:?}"))),
    }
  }
} 

impl TryFromValue for chrono::naive::NaiveDate {
  fn try_from(v: Value) -> Result<Self> {
    match v {
      Value::Text(v)      => Ok(chrono::naive::NaiveDate::parse_from_str(v.as_str(), "%Y-%m-%d")?),
#[cfg(feature = "mysql")]
      Value::MysqlValue(v) => Ok(::mysql::from_value_opt(v)?),
      _ => Err(Error::InvalidTypeForFrom("NaiveDate".to_string(), format!("{v:?}"))),
    }
  }
}

impl TryFromValue for chrono::naive::NaiveDateTime {
  fn try_from(v: Value) -> Result<Self> {
    match v {
#[cfg(feature = "mysql")]
      Value::MysqlValue(v) => Ok(::mysql::from_value_opt(v)?),
      _ => Err(Error::InvalidTypeForFrom("NaiveDateTime".to_string(), format!("{v:?}"))),
    }
  }
}


impl<T> TryFromValue for Option<T> 
where T: TryFromValue,
{
  fn try_from(v: Value) -> Result<Self> { 
    match v {
      Value::Null    => Ok(None),
#[cfg(feature = "mysql")]
      Value::MysqlValue(::mysql::Value::NULL) => Ok(None),
      _              => Ok(Some(TryFromValue::try_from(v)?)),
    }
  }
} 

/// Return a row of results that can be queried to be converted into objects
pub trait Row {
  fn get_value(&self, i: usize)  -> Option<Result<Value>>;

  fn get<T>(&self, i: usize) -> Option<Result<T>>
  where T: TryFromValue,
  {
    self.get_value(i)
    .map(|v| v.and_then(|r| TryFromValue::try_from(r) ) )
  }
}

pub trait TryFromRefRow<R> 
where R: Row,
{
  fn try_from(r: &R) -> Result<Self> where Self: Sized;
}

