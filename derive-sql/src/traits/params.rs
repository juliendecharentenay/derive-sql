use super::*;

pub trait Params {
  fn as_vec_params(&self) -> Result<Vec<Param>>;
}

impl Params for () {
  fn as_vec_params(&self) -> Result<Vec<Param>> { Ok(Vec::new()) }
}
impl<A> Params for A where A: ToParam, {
  fn as_vec_params(&self) -> Result<Vec<Param>> { Ok(vec![self.to_param()?]) }
}
impl<A, B> Params 
for (A, B) 
where A: ToParam, B: ToParam, 
{
  fn as_vec_params(&self) -> Result<Vec<Param>> { Ok(vec![self.0.to_param()?, self.1.to_param()?]) }
}
impl<A, B, C> Params 
for (A, B, C) 
where A: ToParam, B: ToParam, C: ToParam, 
{
  fn as_vec_params(&self) -> Result<Vec<Param>> { Ok(vec![self.0.to_param()?, self.1.to_param()?, self.2.to_param()?]) }
}
impl<A, B, C, D> Params 
for (A, B, C, D) 
where A: ToParam, B: ToParam, C: ToParam, D: ToParam, 
{
  fn as_vec_params(&self) -> Result<Vec<Param>> { Ok(vec![self.0.to_param()?, self.1.to_param()?, self.2.to_param()?, self.3.to_param()?]) }
}
impl<A, B, C, D, E> Params 
for (A, B, C, D, E) 
where A: ToParam, B: ToParam, C: ToParam, D: ToParam, E: ToParam, 
{
  fn as_vec_params(&self) -> Result<Vec<Param>> { Ok(vec![self.0.to_param()?, self.1.to_param()?, self.2.to_param()?, self.3.to_param()?, self.4.to_param()?]) }
}
impl<A, B, C, D, E, F> Params 
for (A, B, C, D, E, F) 
where A: ToParam, B: ToParam, C: ToParam, D: ToParam, E: ToParam, F: ToParam, {
  fn as_vec_params(&self) -> Result<Vec<Param>> { Ok(vec![self.0.to_param()?, self.1.to_param()?, self.2.to_param()?, self.3.to_param()?, self.4.to_param()?, self.5.to_param()?]) }
}
impl<A, B, C, D, E, F, G> Params 
for (A, B, C, D, E, F, G) 
where A: ToParam, B: ToParam, C: ToParam, D: ToParam, E: ToParam, F: ToParam, G: ToParam, {
  fn as_vec_params(&self) -> Result<Vec<Param>> { Ok(vec![self.0.to_param()?, self.1.to_param()?, self.2.to_param()?, self.3.to_param()?, self.4.to_param()?, self.5.to_param()?, self.6.to_param()?]) }
}
impl<A, B, C, D, E, F, G, H> Params 
for (A, B, C, D, E, F, G, H) 
where A: ToParam, B: ToParam, C: ToParam, D: ToParam, E: ToParam, F: ToParam, G: ToParam, H: ToParam, {
  fn as_vec_params(&self) -> Result<Vec<Param>> { Ok(vec![self.0.to_param()?, self.1.to_param()?, self.2.to_param()?, self.3.to_param()?, self.4.to_param()?, self.5.to_param()?, self.6.to_param()?, self.7.to_param()?]) }
}
impl<A, B, C, D, E, F, G, H, I> Params 
for (A, B, C, D, E, F, G, H, I) 
where A: ToParam, B: ToParam, C: ToParam, D: ToParam, E: ToParam, F: ToParam, G: ToParam, H: ToParam, I: ToParam, {
  fn as_vec_params(&self) -> Result<Vec<Param>> { Ok(vec![self.0.to_param()?, self.1.to_param()?, self.2.to_param()?, self.3.to_param()?, self.4.to_param()?, self.5.to_param()?, self.6.to_param()?, self.7.to_param()?, self.8.to_param()?]) }
}
impl<A, B, C, D, E, F, G, H, I, J> Params 
for (A, B, C, D, E, F, G, H, I, J) 
where A: ToParam, B: ToParam, C: ToParam, D: ToParam, E: ToParam, F: ToParam, G: ToParam, H: ToParam, I: ToParam, J: ToParam, {
  fn as_vec_params(&self) -> Result<Vec<Param>> { Ok(vec![self.0.to_param()?, self.1.to_param()?, self.2.to_param()?, self.3.to_param()?, self.4.to_param()?, self.5.to_param()?, self.6.to_param()?, self.7.to_param()?, self.8.to_param()?, self.9.to_param()?]) }
}
impl<A, B, C, D, E, F, G, H, I, J, K> Params 
for (A, B, C, D, E, F, G, H, I, J, K) 
where A: ToParam, B: ToParam, C: ToParam, D: ToParam, E: ToParam, F: ToParam, G: ToParam, H: ToParam, I: ToParam, J: ToParam, K: ToParam, {
  fn as_vec_params(&self) -> Result<Vec<Param>> { Ok(vec![self.0.to_param()?, self.1.to_param()?, self.2.to_param()?, self.3.to_param()?, self.4.to_param()?, self.5.to_param()?, self.6.to_param()?, self.7.to_param()?, self.8.to_param()?, self.9.to_param()?, self.10.to_param()?]) }
}


#[derive(Debug)]
pub enum Param {
  Null,
  Bytes(Vec<u8>),
  SmallInt(i16),
  Int(i32),
  BigInt(i64),
  Real(f32),
  Double(f64),
  Text(String),
  NaiveDate(chrono::naive::NaiveDate),
  NaiveDateTime(chrono::naive::NaiveDateTime),
  Bool(bool),
}

#[cfg(feature = "postgres")]
impl ::postgres::types::ToSql for Param {
  fn to_sql(&self, ty: &::postgres::types::Type, out: &mut bytes::BytesMut) -> std::result::Result<::postgres::types::IsNull, Box<dyn std::error::Error + Sync + Send>> 
  where Self: Sized
  {
    match self {
      Param::Null         => Ok(::postgres::types::IsNull::Yes),
      Param::Bytes(v)     => v.to_sql(ty, out),
      Param::SmallInt(v)  => v.to_sql(ty, out),
      Param::Int(v)       => v.to_sql(ty, out),
      Param::BigInt(v)    => v.to_sql(ty, out),
      Param::Real(v)      => v.to_sql(ty, out),
      Param::Double(v)    => v.to_sql(ty, out),
      Param::Text(v)      => v.to_sql(ty, out),
      Param::NaiveDate(v) => v.to_sql(ty, out),
      Param::NaiveDateTime(v) => v.to_sql(ty, out),
      Param::Bool(v)      => v.to_sql(ty, out),
    }
  }

  fn accepts(ty: &::postgres::types::Type) -> bool { 
    match ty {
      _ => true,
    }
  }

  fn to_sql_checked(&self, ty: &::postgres::types::Type, out: &mut bytes::BytesMut) -> std::result::Result<::postgres::types::IsNull, Box<dyn std::error::Error + Send + Sync>> { 
    match self {
      Param::Null         => Ok(::postgres::types::IsNull::Yes),
      Param::Bytes(v)     => v.to_sql_checked(ty, out),
      Param::SmallInt(v)  => v.to_sql_checked(ty, out),
      Param::Int(v)       => v.to_sql_checked(ty, out),
      Param::BigInt(v)    => v.to_sql_checked(ty, out),
      Param::Real(v)      => v.to_sql_checked(ty, out),
      Param::Double(v)    => v.to_sql_checked(ty, out),
      Param::Text(v)      => v.to_sql_checked(ty, out),
      Param::NaiveDate(v) => v.to_sql_checked(ty, out),
      Param::NaiveDateTime(v) => v.to_sql_checked(ty, out),
      Param::Bool(v)      => v.to_sql_checked(ty, out),
    }
  }
}

#[cfg(feature = "sqlite")]
impl ::rusqlite::types::ToSql for Param {
  fn to_sql(&self) -> ::rusqlite::Result<::rusqlite::types::ToSqlOutput<'_>> {
    match self {
      Param::Null      => ::rusqlite::types::Null.to_sql(),
      Param::Bytes(v)  => v.to_sql(),
      Param::SmallInt(v) => v.to_sql(),
      Param::Int(v)    => v.to_sql(),
      Param::BigInt(v) => v.to_sql(),
      Param::Real(v)   => v.to_sql(),
      Param::Double(v) => v.to_sql(),
      Param::Text(v)   => v.to_sql(),
      Param::NaiveDate(v) => v.to_sql(),
      Param::NaiveDateTime(v) => v.to_sql(),
      Param::Bool(v)   => v.to_sql(),
    }
  }
}

#[cfg(feature = "mysql")]
impl TryFrom<Param> for ::mysql::Value {
  type Error = Error;
  fn try_from(p: Param) -> std::result::Result<Self, Self::Error> {
    match p {
      Param::Null      => Ok(::mysql::Value::NULL),
      Param::Bytes(v)  => Ok(v.into()),
      Param::SmallInt(v) => Ok(v.into()),
      Param::Int(v)    => Ok(v.into()),
      Param::BigInt(v) => Ok(v.into()),
      Param::Real(v)   => Ok(v.into()),
      Param::Double(v) => Ok(v.into()),
      Param::Text(v)   => Ok(v.into()),
      Param::NaiveDate(v) => Ok(v.into()),
      Param::NaiveDateTime(v) => Ok(v.into()),
      Param::Bool(v)   => Ok((if v { 1 } else { 0 }).into()),
    }
  }
}

pub trait ToParam        { fn to_param(&self) -> Result<Param>; }
impl ToParam for Vec<u8> { fn to_param(&self) -> Result<Param> { Ok(Param::Bytes(self.clone())) } }
// impl ToParam for bool    { fn to_param(&self) -> Result<Param> { Ok(Param::Int(if *self { 1 } else { 0 })) } }
impl ToParam for bool    { fn to_param(&self) -> Result<Param> { Ok(Param::Bool((*self).into())) } }
impl ToParam for usize   { fn to_param(&self) -> Result<Param> { Ok(Param::BigInt((*self).try_into()?)) } }
impl ToParam for u8      { fn to_param(&self) -> Result<Param> { Ok(Param::SmallInt((*self).into())) } }
impl ToParam for u16     { fn to_param(&self) -> Result<Param> { Ok(Param::SmallInt((*self).try_into()?)) } }
impl ToParam for u32     { fn to_param(&self) -> Result<Param> { Ok(Param::Int((*self).try_into()?)) } }
impl ToParam for u64     { fn to_param(&self) -> Result<Param> { Ok(Param::BigInt((*self).try_into()?)) } }
impl ToParam for isize   { fn to_param(&self) -> Result<Param> { Ok(Param::BigInt((*self).try_into()?)) } }
impl ToParam for i8      { fn to_param(&self) -> Result<Param> { Ok(Param::SmallInt((*self).into())) } }
impl ToParam for i16     { fn to_param(&self) -> Result<Param> { Ok(Param::SmallInt((*self).into())) } }
impl ToParam for i32     { fn to_param(&self) -> Result<Param> { Ok(Param::Int((*self).into())) } }
impl ToParam for i64     { fn to_param(&self) -> Result<Param> { Ok(Param::BigInt(*self)) } }
impl ToParam for f32     { fn to_param(&self) -> Result<Param> { Ok(Param::Real((*self).into())) } }
impl ToParam for f64     { fn to_param(&self) -> Result<Param> { Ok(Param::Double(*self)) } }
impl ToParam for String  { fn to_param(&self) -> Result<Param> { Ok(Param::Text(self.clone())) } }
impl ToParam for chrono::naive::NaiveDate { fn to_param(&self) -> Result<Param> { Ok(Param::NaiveDate(self.clone())) } }
impl ToParam for chrono::naive::NaiveDateTime { fn to_param(&self) -> Result<Param> { Ok(Param::NaiveDateTime(self.clone())) } }

impl<T> ToParam for Option<T>
where T: ToParam
{
  fn to_param(&self) -> Result<Param> {
    if let Some(v) = self {
      v.to_param()
    } else {
      Ok(Param::Null)
    }
  }
}

