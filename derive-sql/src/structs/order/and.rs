//! Implement `AND` combination of filter.
use super::*;

pub struct And<T, U, V, W, X, Y>
where T: traits::FlavoredOrder,
      U: traits::FlavoredOrder,
      V: traits::FlavoredOrder,
      W: traits::FlavoredOrder,
      X: traits::FlavoredOrder,
      Y: traits::FlavoredOrder,
{
  t: T,
  u: U,
  v: Option<V>,
  w: Option<W>,
  x: Option<X>,
  y: Option<Y>,
}

impl<T, U> std::convert::From<(T, U)> for And<T, U, T, T, T, T>
where T: traits::FlavoredOrder, U: traits::FlavoredOrder {
  fn from((t, u): (T, U)) -> Self {
    And { t, u, v: None, w: None, x: None, y: None }
  }
}

impl<T, U, V> std::convert::From<(T, U, V)> for And<T, U, V, T, T, T>
where T: traits::FlavoredOrder, U: traits::FlavoredOrder, V: traits::FlavoredOrder {
  fn from((t, u, v): (T, U, V)) -> Self {
    And { t, u, v: Some(v), w: None, x: None, y: None }
  }
}

impl<T, U, V, W> std::convert::From<(T, U, V, W)> for And<T, U, V, W, T, T>
where T: traits::FlavoredOrder, U: traits::FlavoredOrder, V: traits::FlavoredOrder, W: traits::FlavoredOrder {
  fn from((t, u, v, w): (T, U, V, W)) -> Self {
    And { t, u, v: Some(v), w: Some(w), x: None, y: None }
  }
}

impl<T, U, V, W, X> std::convert::From<(T, U, V, W, X)> for And<T, U, V, W, X, T>
where T: traits::FlavoredOrder, U: traits::FlavoredOrder, V: traits::FlavoredOrder, W: traits::FlavoredOrder, X: traits::FlavoredOrder {
  fn from((t, u, v, w, x): (T, U, V, W, X)) -> Self {
    And { t, u, v: Some(v), w: Some(w), x: Some(x), y: None }
  }
}

impl<T, U, V, W, X, Y> std::convert::From<(T, U, V, W, X, Y)> for And<T, U, V, W, X, Y>
where T: traits::FlavoredOrder, U: traits::FlavoredOrder, V: traits::FlavoredOrder, W: traits::FlavoredOrder, X: traits::FlavoredOrder, Y: traits::FlavoredOrder {
  fn from((t, u, v, w, x, y): (T, U, V, W, X, Y)) -> Self {
    And { t, u, v: Some(v), w: Some(w), x: Some(x), y: Some(y) }
  }
}

impl<T, U, V, W, X, Y> traits::FlavoredOrder for And<T, U, V, W, X, Y>
where T: traits::FlavoredOrder, U: traits::FlavoredOrder, V: traits::FlavoredOrder,
      W: traits::FlavoredOrder, X: traits::FlavoredOrder, Y: traits::FlavoredOrder,
{
  fn as_order_clause<C, R>(&self, conn: &C) -> Result<String> 
  where C: traits::Connection<R>, R: traits::Row
  {
    let a = self.t.as_order_clause(conn)?; let b = self.u.as_order_clause(conn)?;
    let r = match self.v.as_ref().map(|v| v.as_order_clause(conn)).transpose()? {
      Some(c) => match self.w.as_ref().map(|w| w.as_order_clause(conn)).transpose()? {
        Some(d) => match self.x.as_ref().map(|x| x.as_order_clause(conn)).transpose()? {
          Some(e) => match self.y.as_ref().map(|y| y.as_order_clause(conn)).transpose()? {
            Some(f) => format!("( {a}, {b}, {c}, {d}, {e}, {f} )"),
            None => format!("( {a}, {b}, {c}, {d}, {e} )"),
          },
          None => format!("( {a}, {b}, {c}, {d} )"),
        },
        None => format!("( {a}, {b}, {c} )"),
      },
      None => format!("( {a}, {b} )"),
    };
    Ok(r)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct WordOne { }
  impl traits::FlavoredOrder for WordOne {
    fn as_order_clause<C, R>(&self, _: &C) -> Result<String>
    where C: traits::Connection<R>, R: traits::Row,
    {
      Ok("1".to_string())
    }
  }

  struct WordHouse { }
  impl traits::FlavoredOrder for WordHouse {
    fn as_order_clause<C, R>(&self, _: &C) -> Result<String>
    where C: traits::Connection<R>, R: traits::Row,
    {
      Ok("House".to_string())
    }
  }

  #[cfg(test)]
  fn it_combines_as_order_clause() -> DeriveSqlResult<()> {
    use traits::FlavoredOrder;
    let conn = traits::tests::SQLiteFlavoredConnection {};
    type Row = traits::tests::Row;

    let and: And<_, _, _, _, _, _> = (WordOne {}, WordOne {}).into();
    assert!(and.as_order_clause::<_, Row>(&conn)?.eq("( 1, 1 )"));

    let and: And<_, _, _, _, _, _> = (WordOne {}, WordHouse {}, WordOne {}).into();
    assert!(and.as_order_clause::<_, Row>(&conn)?.eq("( 1, House, 1 )"));

    let and: And<_, _, _, _, _, _> = (WordOne {}, WordHouse {}, WordHouse {}, WordOne {}).into();
    assert!(and.as_order_clause::<_, Row>(&conn)?.eq("( 1, House, House, 1 )"));

    let and: And<_, _, _, _, _, _> = (WordOne {}, WordHouse {}, WordHouse {}, WordOne {}, WordHouse {}, ).into();
    assert!(and.as_order_clause::<_, Row>(&conn)?.eq("( 1, House, House, 1, House )"));

    let and: And<_, _, _, _, _, _> = (WordOne {}, WordHouse {}, WordHouse {}, WordOne {}, WordHouse {}, WordHouse {}).into();
    assert!(and.as_order_clause::<_, Row>(&conn)?.eq("( 1, House, House, 1, House, House )"));

    Ok(())
  }
}
