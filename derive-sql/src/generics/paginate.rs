use super::*;

pub struct Paginate<T, I>
where T: traits::AsStatement<I>,
{
  inner: T,
  limit: Option<usize>,
  offset: Option<usize>,
  phantom_i: std::marker::PhantomData<I>,
}

impl<T, I> Paginate<T, I>
where T: traits::AsStatement<I>,
{
  pub fn with_limit(mut self, limit: usize) -> Self { self.limit = Some(limit); self }
  pub fn with_offset(mut self, offset: usize) -> Self { self.offset = Some(offset); self }
}

impl<T, I> std::convert::From<T> for Paginate<T, I>
where T: traits::AsStatement<I>,
{
  fn from(v: T) -> Self {
    Paginate {
      inner: v,
      limit: None,
      offset: None,
      phantom_i: std::marker::PhantomData::default(),
    }
  }
}

impl<I, T> traits::AsStatement<I> for Paginate<T, I>
where T: traits::AsStatement<I>,
{
  fn as_statement(&self) -> DeriveSqlResult<String> {
    Ok(format!("{statement} {limit} {offset}",
      statement = self.inner.as_statement()?,
      limit = self.limit.as_ref().map(|v| format!("LIMIT {v}")).unwrap_or_default(),
      offset = self.offset.as_ref().map(|v| format!("OFFSET {v}")).unwrap_or_default(),
    ))
  }
}

impl<I, T> traits::IsSelect for Paginate<T, I>
where T: traits::AsStatement<I> + traits::IsSelect,
{}

