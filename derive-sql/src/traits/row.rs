use super::*;

#[derive(Debug, Clone)]
pub enum Value {
  Null,
  Bool(bool),
  Integer(i64),
  UInteger(u64),
  Real(f64),
  Text(String),
  Blob(Vec<u8>),
#[cfg(feature = "mysql")]
  MysqlValue(::mysql::Value),
}


#[cfg(feature = "postgres")]
impl<'a> postgres::types::FromSql<'a> for Value {
  fn from_sql(ty: &postgres::types::Type, raw: &'a [u8]) -> std::result::Result<Self, Box<dyn core::error::Error + core::marker::Sync + core::marker::Send>> {
    use postgres::types::Type;
    let name = ty.name();
    match true {
      true if raw.len() == 0 => Ok(Value::Null),
      true if name.eq(Type::BOOL.name()) => Ok(Value::Bool(<bool as postgres::types::FromSql>::from_sql(ty, raw)?)),
      // true if name.eq(Type::DATE.name()) => true,
      true if name.eq(Type::FLOAT4.name()) => Ok(Value::Real(<f32 as postgres::types::FromSql>::from_sql(ty, raw)?.into())),
      true if name.eq(Type::FLOAT8.name()) => Ok(Value::Real(<f64 as postgres::types::FromSql>::from_sql(ty, raw)?.into())),
      true if name.eq(Type::INT2.name()) => Ok(Value::Integer(<i16 as postgres::types::FromSql>::from_sql(ty, raw)?.into())),
      true if name.eq(Type::INT4.name()) => Ok(Value::Integer(<i32 as postgres::types::FromSql>::from_sql(ty, raw)?.into())),
      true if name.eq(Type::INT8.name()) => Ok(Value::Integer(<i64 as postgres::types::FromSql>::from_sql(ty, raw)?.into())),
      true if name.eq(Type::TEXT.name()) => Ok(Value::Text(<String as postgres::types::FromSql>::from_sql(ty, raw)?.into())),
      true if name.eq(Type::BYTEA.name()) => Ok(Value::Blob(<Vec<u8> as postgres::types::FromSql>::from_sql(ty, raw)?.into())),
      _ => Err(Error::PostgreSQLInvalidConversion(name.to_string()).into()),
    }
  }

  fn accepts(ty: &postgres::types::Type) -> bool {
    use postgres::types::Type;
    let name = ty.name();
    match true {
      true if name.eq(Type::BOOL.name()) => true,
      // true if name.eq(Type::DATE.name()) => true,
      true if name.eq(Type::FLOAT4.name()) => true,
      true if name.eq(Type::FLOAT8.name()) => true,
      true if name.eq(Type::INT4.name()) => true,
      true if name.eq(Type::INT8.name()) => true,
      true if name.eq(Type::TEXT.name()) => true,
      true if name.eq(Type::BYTEA.name()) => true,
      _ => false,
    }
  }
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
      Value::Bool(v) => Ok(v),
      Value::Integer(v) => Ok(v != 0),
#[cfg(feature = "mysql")]
      Value::MysqlValue(v) => Ok(::mysql::from_value_opt(v)?),
      _ => Err(Error::InvalidTypeFor("bool".to_string())),
    }
  }
} 

impl TryFromValue for usize {
  fn try_from(v: Value) -> Result<Self> { 
    match v {
      Value::UInteger(v) => Ok(v.try_into()?),
      Value::Integer(v)  => Ok(v.try_into()?),
#[cfg(feature = "mysql")]
      Value::MysqlValue(v) => Ok(::mysql::from_value_opt(v)?),
      _ => Err(Error::InvalidTypeForFrom("usize".to_string(), format!("{v:?}"))),
    }
  }
} 

impl TryFromValue for u8 {
  fn try_from(v: Value) -> Result<Self> { 
    match v {
      Value::UInteger(v) => Ok(v.try_into()?),
      Value::Integer(v)  => Ok(v.try_into()?),
#[cfg(feature = "mysql")]
      Value::MysqlValue(v) => Ok(::mysql::from_value_opt(v)?),
      _ => Err(Error::InvalidTypeForFrom("u8".to_string(), format!("{v:?}"))),
    }
  }
} 

impl TryFromValue for u16 {
  fn try_from(v: Value) -> Result<Self> { 
    match v {
      Value::UInteger(v) => Ok(v.try_into()?),
      Value::Integer(v)  => Ok(v.try_into()?),
#[cfg(feature = "mysql")]
      Value::MysqlValue(v) => Ok(::mysql::from_value_opt(v)?),
      _ => Err(Error::InvalidTypeForFrom("u16".to_string(), format!("{v:?}"))),
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

impl TryFromValue for u64 {
  fn try_from(v: Value) -> Result<Self> { 
    match v {
      Value::UInteger(v) => Ok(v),
      Value::Integer(v)  => Ok(v.try_into()?),
#[cfg(feature = "mysql")]
      Value::MysqlValue(v) => Ok(::mysql::from_value_opt(v)?),
      _ => Err(Error::InvalidTypeForFrom("u64".to_string(), format!("{v:?}"))),
    }
  }
} 

impl TryFromValue for isize {
  fn try_from(v: Value) -> Result<Self> { 
    match v {
      Value::Integer(v) => Ok(v.try_into()?),
#[cfg(feature = "mysql")]
      Value::MysqlValue(v) => Ok(::mysql::from_value_opt(v)?),
      _ => Err(Error::InvalidTypeFor("isize".to_string())),
    }
  }
}

impl TryFromValue for i8 {
  fn try_from(v: Value) -> Result<Self> { 
    match v {
      Value::Integer(v) => Ok(v.try_into()?),
#[cfg(feature = "mysql")]
      Value::MysqlValue(v) => Ok(::mysql::from_value_opt(v)?),
      _ => Err(Error::InvalidTypeFor("i8".to_string())),
    }
  }
}

impl TryFromValue for i16 {
  fn try_from(v: Value) -> Result<Self> { 
    match v {
      Value::Integer(v) => Ok(v.try_into()?),
#[cfg(feature = "mysql")]
      Value::MysqlValue(v) => Ok(::mysql::from_value_opt(v)?),
      _ => Err(Error::InvalidTypeFor("i16".to_string())),
    }
  }
}

impl TryFromValue for i32 {
  fn try_from(v: Value) -> Result<Self> { 
    match v {
      Value::Integer(v) => Ok(v.try_into()?),
#[cfg(feature = "mysql")]
      Value::MysqlValue(v) => Ok(::mysql::from_value_opt(v)?),
      _ => Err(Error::InvalidTypeFor("i32".to_string())),
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

impl TryFromValue for f32 {
  fn try_from(v: Value) -> Result<Self> { 
    match v {
      Value::Real(v) => Ok(v as f32),
#[cfg(feature = "mysql")]
      Value::MysqlValue(v) => Ok(::mysql::from_value_opt(v)?),
      _ => Err(Error::InvalidTypeFor("f32".to_string())),
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

/// Trait to be implemented to allow a `Row` to be converted into an object
pub trait TryFromRefRow<R> 
where R: Row,
{
  fn try_from(r: &R) -> Result<Self> where Self: Sized;
}

impl<R, T> TryFromRefRow<R> for T
where R: Row,
      T: TryFromValue + Sized,
{
  fn try_from(r: &R) -> Result<Self> {
    Ok(r.get(0).ok_or(Error::RowItemNotFound(0))??)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_retrieves_a_string() -> Result<()> {
    struct MyRow {}
    impl Row for MyRow { fn get_value(&self, _i: usize) -> Option<Result<Value>> { Some(Ok(Value::Text(format!("hello")))) } }
    let r: String = <String as TryFromRefRow<_>>::try_from(&MyRow {})?;
    assert!(r.eq("hello"));

    struct MyRow2 {}
    impl Row for MyRow2 { fn get_value(&self, _i: usize) -> Option<Result<Value>> { None } }
    assert!(<String as TryFromRefRow<_>>::try_from(&MyRow2 {}).is_err());

    Ok(())
  }
}
