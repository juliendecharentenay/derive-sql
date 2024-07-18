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


pub enum Param {
  Null,
  Bytes(Vec<u8>),
  Int(i64),
  Double(f64),
  Text(String),
  NaiveDate(chrono::naive::NaiveDate),
  NaiveDateTime(chrono::naive::NaiveDateTime),
}

#[cfg(feature = "sqlite")]
impl ::rusqlite::types::ToSql for Param {
  fn to_sql(&self) -> ::rusqlite::Result<::rusqlite::types::ToSqlOutput<'_>> {
    match self {
      Param::Null      => ::rusqlite::types::Null.to_sql(),
      Param::Bytes(v)  => v.to_sql(),
      Param::Int(v)    => v.to_sql(),
      Param::Double(v) => v.to_sql(),
      Param::Text(v)   => v.to_sql(),
      Param::NaiveDate(v) => v.to_sql(),
      Param::NaiveDateTime(v) => v.to_sql(),
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
      Param::Int(v)    => Ok(v.into()),
      Param::Double(v) => Ok(v.into()),
      Param::Text(v)   => Ok(v.into()),
      Param::NaiveDate(v) => Ok(v.into()),
      Param::NaiveDateTime(v) => Ok(v.into()),
    }
  }
}

pub trait ToParam        { fn to_param(&self) -> Result<Param>; }
impl ToParam for Vec<u8> { fn to_param(&self) -> Result<Param> { Ok(Param::Bytes(self.clone())) } }
impl ToParam for bool    { fn to_param(&self) -> Result<Param> { Ok(Param::Int(if *self { 1 } else { 0 })) } }
impl ToParam for u32     { fn to_param(&self) -> Result<Param> { Ok(Param::Int((*self).into())) } }
impl ToParam for i64     { fn to_param(&self) -> Result<Param> { Ok(Param::Int(*self)) } }
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
  

