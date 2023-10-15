//! Convenience struct implementing trait `Sqlable` for testing purposes
use super::*;

pub struct MockSqlable<T,E,S> 
{
  pub on_count: Option<Box<dyn Fn() -> usize>>,
  pub count_calls: std::cell::RefCell<Vec<S>>,

  pub on_select: Option<Box<dyn Fn() -> Vec<T>>>,
  pub select_calls: std::cell::RefCell<Vec<S>>,

  pub on_insert: Option<Box<dyn Fn() -> T>>,
  pub insert_calls: std::cell::RefCell<Vec<T>>,

  pub on_update: Option<Box<dyn Fn() -> T>>,
  pub update_calls: std::cell::RefCell<Vec<(S, T)>>,

  pub delete_calls: std::cell::RefCell<Vec<S>>,

  pub delete_table_calls: std::cell::RefCell<Vec<()>>,

  // phantom_data: std::marker::PhantomData<T>,
  phantom_error: std::marker::PhantomData<E>,
  // phantom_selector: std::marker::PhantomData<S>,
}

impl<T,E,S> Default for MockSqlable<T,E,S> {
  fn default() -> Self {
    MockSqlable::<T,E,S> {
      on_count: None,
      count_calls: std::cell::RefCell::new(Vec::new()),
      on_select: None,
      select_calls: std::cell::RefCell::new(Vec::new()),
      on_insert: None,
      insert_calls: std::cell::RefCell::new(Vec::new()),
      on_update: None,
      update_calls: std::cell::RefCell::new(Vec::new()),
      delete_calls: std::cell::RefCell::new(Vec::new()),
      delete_table_calls: std::cell::RefCell::new(Vec::new()),
      phantom_error: std::marker::PhantomData::default(),
    }
  }
}

impl<T,E,S> MockSqlable<T,E,S> 
{
  pub fn with_on_count(mut self, f: Box<dyn Fn() -> usize>) -> MockSqlable<T,E,S> {
    self.on_count = Some(f); self
  }
  pub fn with_on_select(mut self, f: Box<dyn Fn() -> Vec<T>>) -> MockSqlable<T,E,S> {
    self.on_select = Some(f); self
  }
  pub fn with_on_insert(mut self, f: Box<dyn Fn() -> T>) -> MockSqlable<T,E,S> {
    self.on_insert = Some(f); self
  }
  pub fn with_on_update(mut self, f: Box<dyn Fn() -> T>) -> MockSqlable<T,E,S> {
    self.on_update = Some(f); self
  }
}

impl<T,E,S> Sqlable for MockSqlable<T,E,S> 
where E: std::convert::From<&'static str>
{
  type Item = T;
  type Error = E;
  type Selector = S;

  fn count(&self, s: Self::Selector) -> Result<usize, Self::Error> {
    self.count_calls.borrow_mut().push(s);
    Ok(self.on_count.as_ref().map(|f| f()).ok_or("Count call did not succeed")?)
  }

  fn select(&self, s: Self::Selector) -> Result<Vec<Self::Item>, Self::Error> {
    self.select_calls.borrow_mut().push(s);
    Ok(self.on_select.as_ref().map(|f| f()).ok_or("Select call did not succeed")?)
  }

  fn insert(&mut self, item: Self::Item) -> Result<Self::Item, Self::Error> {
    self.insert_calls.borrow_mut().push(item);
    Ok(self.on_insert.as_ref().map(|f| f()).ok_or("Insert call did not succeed")?)
  }

  fn update(&mut self, s: Self::Selector, item: Self::Item) -> Result<Self::Item, Self::Error> {
    self.update_calls.borrow_mut().push((s, item));
    Ok(self.on_update.as_ref().map(|f| f()).ok_or("Update call did not succeed")?)
  }

  fn delete(&mut self, s: Self::Selector) -> Result<(), Self::Error> {
    self.delete_calls.borrow_mut().push(s); Ok(())
  }

  fn delete_table(&mut self) -> Result<(), Self::Error> {
    self.delete_table_calls.borrow_mut().push(()); Ok(())
  }
}

