//! Implement `OR` combination of filter
use super::*;

pub struct Or<T, U, V, W, X, Y>
where T: FilterTrait,
      U: FilterTrait,
      V: FilterTrait,
      W: FilterTrait,
      X: FilterTrait,
      Y: FilterTrait,
{
  t: T,
  u: U,
  v: Option<V>,
  w: Option<W>,
  x: Option<X>,
  y: Option<Y>,
}

impl<T, U> std::convert::From<(T, U)> for Or<T, U, T, T, T, T>
where T: FilterTrait, U: FilterTrait {
  fn from((t, u): (T, U)) -> Self {
    Or { t, u, v: None, w: None, x: None, y: None }
  }
}

impl<T, U, V> std::convert::From<(T, U, V)> for Or<T, U, V, T, T, T>
where T: FilterTrait, U: FilterTrait, V: FilterTrait {
  fn from((t, u, v): (T, U, V)) -> Self {
    Or { t, u, v: Some(v), w: None, x: None, y: None }
  }
}

impl<T, U, V, W> std::convert::From<(T, U, V, W)> for Or<T, U, V, W, T, T>
where T: FilterTrait, U: FilterTrait, V: FilterTrait, W: FilterTrait {
  fn from((t, u, v, w): (T, U, V, W)) -> Self {
    Or { t, u, v: Some(v), w: Some(w), x: None, y: None }
  }
}

impl<T, U, V, W, X> std::convert::From<(T, U, V, W, X)> for Or<T, U, V, W, X, T>
where T: FilterTrait, U: FilterTrait, V: FilterTrait, W: FilterTrait, X: FilterTrait {
  fn from((t, u, v, w, x): (T, U, V, W, X)) -> Self {
    Or { t, u, v: Some(v), w: Some(w), x: Some(x), y: None }
  }
}

impl<T, U, V, W, X, Y> std::convert::From<(T, U, V, W, X, Y)> for Or<T, U, V, W, X, Y>
where T: FilterTrait, U: FilterTrait, V: FilterTrait, W: FilterTrait, X: FilterTrait, Y: FilterTrait {
  fn from((t, u, v, w, x, y): (T, U, V, W, X, Y)) -> Self {
    Or { t, u, v: Some(v), w: Some(w), x: Some(x), y: Some(y) }
  }
}

impl<T, U, V, W, X, Y> FilterTrait for Or<T, U, V, W, X, Y>
where T: FilterTrait, U: FilterTrait, V: FilterTrait,
      W: FilterTrait, X: FilterTrait, Y: FilterTrait,
{
  fn filter(&self) -> String {
    let a = self.t.filter(); let b = self.u.filter();
    match self.v.as_ref().map(|v| v.filter()) {
      Some(c) => match self.w.as_ref().map(|w| w.filter()) {
        Some(d) => match self.x.as_ref().map(|x| x.filter()) {
          Some(e) => match self.y.as_ref().map(|y| y.filter()) {
            Some(f) => format!("( {a} OR {b} OR {c} OR {d} OR {e} OR {f} )"),
            None => format!("( {a} OR {b} OR {c} OR {d} OR {e} )"),
          },
          None => format!("( {a} OR {b} OR {c} OR {d} )"),
        },
        None => format!("( {a} OR {b} OR {c} )"),
      },
      None => format!("( {a} OR {b} )"),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct WordOne { }
  impl FilterTrait for WordOne {
    fn filter(&self) -> String {
      "1".to_string()
    }
  }

  struct WordHouse { }
  impl FilterTrait for WordHouse {
    fn filter(&self) -> String {
      "House".to_string()
    }
  }

  #[test]
  fn it_combines_filter() -> DeriveSqlResult<()> {
    let and: Or<_, _, _, _, _, _> = (WordOne {}, WordOne {}).into();
    assert!(and.filter().eq("( 1 OR 1 )"));

    let and: Or<_, _, _, _, _, _> = (WordOne {}, WordHouse {}, WordOne {}).into();
    assert!(and.filter().eq("( 1 OR House OR 1 )"));

    let and: Or<_, _, _, _, _, _> = (WordOne {}, WordHouse {}, WordHouse {}, WordOne {}).into();
    assert!(and.filter().eq("( 1 OR House OR House OR 1 )"));

    let and: Or<_, _, _, _, _, _> = (WordOne {}, WordHouse {}, WordHouse {}, WordOne {}, WordHouse {}, ).into();
    assert!(and.filter().eq("( 1 OR House OR House OR 1 OR House )"));

    let and: Or<_, _, _, _, _, _> = (WordOne {}, WordHouse {}, WordHouse {}, WordOne {}, WordHouse {}, WordHouse {}).into();
    assert!(and.filter().eq("( 1 OR House OR House OR 1 OR House OR House )"));

    Ok(())
  }
}
