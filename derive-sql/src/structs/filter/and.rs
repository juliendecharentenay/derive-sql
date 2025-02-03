//! Implement `AND` combination of filter.
use super::*;

pub struct And<T, U, V, W, X, Y>
where T: traits::FlavoredFilter,
      U: traits::FlavoredFilter,
      V: traits::FlavoredFilter,
      W: traits::FlavoredFilter,
      X: traits::FlavoredFilter,
      Y: traits::FlavoredFilter,
{
  t: T,
  u: U,
  v: Option<V>,
  w: Option<W>,
  x: Option<X>,
  y: Option<Y>,
}

impl<T, U> std::convert::From<(T, U)> for And<T, U, T, T, T, T>
where T: traits::FlavoredFilter, U: traits::FlavoredFilter {
  fn from((t, u): (T, U)) -> Self {
    And { t, u, v: None, w: None, x: None, y: None }
  }
}

impl<T, U, V> std::convert::From<(T, U, V)> for And<T, U, V, T, T, T>
where T: traits::FlavoredFilter, U: traits::FlavoredFilter, V: traits::FlavoredFilter {
  fn from((t, u, v): (T, U, V)) -> Self {
    And { t, u, v: Some(v), w: None, x: None, y: None }
  }
}

impl<T, U, V, W> std::convert::From<(T, U, V, W)> for And<T, U, V, W, T, T>
where T: traits::FlavoredFilter, U: traits::FlavoredFilter, V: traits::FlavoredFilter, W: traits::FlavoredFilter {
  fn from((t, u, v, w): (T, U, V, W)) -> Self {
    And { t, u, v: Some(v), w: Some(w), x: None, y: None }
  }
}

impl<T, U, V, W, X> std::convert::From<(T, U, V, W, X)> for And<T, U, V, W, X, T>
where T: traits::FlavoredFilter, U: traits::FlavoredFilter, V: traits::FlavoredFilter, W: traits::FlavoredFilter, X: traits::FlavoredFilter {
  fn from((t, u, v, w, x): (T, U, V, W, X)) -> Self {
    And { t, u, v: Some(v), w: Some(w), x: Some(x), y: None }
  }
}

impl<T, U, V, W, X, Y> std::convert::From<(T, U, V, W, X, Y)> for And<T, U, V, W, X, Y>
where T: traits::FlavoredFilter, U: traits::FlavoredFilter, V: traits::FlavoredFilter, W: traits::FlavoredFilter, X: traits::FlavoredFilter, Y: traits::FlavoredFilter {
  fn from((t, u, v, w, x, y): (T, U, V, W, X, Y)) -> Self {
    And { t, u, v: Some(v), w: Some(w), x: Some(x), y: Some(y) }
  }
}

impl<T, U, V, W, X, Y> traits::FlavoredFilter for And<T, U, V, W, X, Y>
where T: traits::FlavoredFilter, U: traits::FlavoredFilter, V: traits::FlavoredFilter,
      W: traits::FlavoredFilter, X: traits::FlavoredFilter, Y: traits::FlavoredFilter,
{
  fn filter<C, R>(&self, conn: &C) -> Result<String>
  where C: traits::Connection<R>, R: traits::Row,
  {
    let a = self.t.filter(conn)?; let b = self.u.filter(conn)?;
    let r = match self.v.as_ref().map(|v| v.filter(conn)).transpose()? {
      Some(c) => match self.w.as_ref().map(|w| w.filter(conn)).transpose()? {
        Some(d) => match self.x.as_ref().map(|x| x.filter(conn)).transpose()? {
          Some(e) => match self.y.as_ref().map(|y| y.filter(conn)).transpose()? {
            Some(f) => format!("( {a} AND {b} AND {c} AND {d} AND {e} AND {f} )"),
            None => format!("( {a} AND {b} AND {c} AND {d} AND {e} )"),
          },
          None => format!("( {a} AND {b} AND {c} AND {d} )"),
        },
        None => format!("( {a} AND {b} AND {c} )"),
      },
      None => format!("( {a} AND {b} )"),
    };
    Ok(r)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct WordOne { }
  impl traits::FlavoredFilter for WordOne {
    fn filter<C, R>(&self, _: &C) -> Result<String>
    where C: traits::Connection<R>, R: traits::Row,
    {
      Ok("1".to_string())
    }
  }

  struct WordHouse { }
  impl traits::FlavoredFilter for WordHouse {
    fn filter<C, R>(&self, _: &C) -> Result<String>
    where C: traits::Connection<R>, R: traits::Row,
    {
      Ok("House".to_string())
    }
  }

  #[cfg(test)]
  fn it_combines_filter() -> Result<()> {
    use traits::FlavoredFilter;
    let conn = traits::tests::SQLiteFlavoredConnection {};
    type Row = traits::tests::Row;

    let and: And<_, _, _, _, _, _> = (WordOne {}, WordOne {}).into();
    assert!(and.filter::<_, Row>(&conn)?.eq("( 1 AND 1 )"));

    let and: And<_, _, _, _, _, _> = (WordOne {}, WordHouse {}, WordOne {}).into();
    assert!(and.filter::<_, Row>(&conn)?.eq("( 1 AND House AND 1 )"));

    let and: And<_, _, _, _, _, _> = (WordOne {}, WordHouse {}, WordHouse {}, WordOne {}).into();
    assert!(and.filter::<_, Row>(&conn)?.eq("( 1 AND House AND House AND 1 )"));

    let and: And<_, _, _, _, _, _> = (WordOne {}, WordHouse {}, WordHouse {}, WordOne {}, WordHouse {}, ).into();
    assert!(and.filter::<_, Row>(&conn)?.eq("( 1 AND House AND House AND 1 AND House )"));

    let and: And<_, _, _, _, _, _> = (WordOne {}, WordHouse {}, WordHouse {}, WordOne {}, WordHouse {}, WordHouse {}).into();
    assert!(and.filter::<_, Row>(&conn)?.eq("( 1 AND House AND House AND 1 AND House AND House )"));

    Ok(())
  }
}
