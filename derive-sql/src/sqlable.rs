/// Definition of `Sqlable` trait to be implemented to allow interaction with SQL tables.
pub trait Sqlable {
  /// Defines the struct corresponding to the items in the SQL table
  type Item;
  /// Defines the struct used for returning errors
  type Error;
  /// Defines the struct used for providing selection, filtering to the implementation of the trait
  type Selector;

  /// Count the number of items in the table meeting the selector requirements
  fn count(&self, s: Self::Selector) -> Result<usize, Self::Error>;
  /// Retrieve the array of items in the table meeting the selector requirements
  fn select(&self, s: Self::Selector) -> Result<Vec<Self::Item>, Self::Error>;
  /// Insert a new item in the table and returned the inserted item.
  fn insert(&mut self, item: Self::Item) -> Result<Self::Item, Self::Error>;
  /// Update items meeting the selector requirements with the values of the provided item. Returns
  /// the array of items modified with their updated value
  fn update(&mut self, s: Self::Selector, item: Self::Item) -> Result<Self::Item, Self::Error>;
  /// Delete items in the table meeting the selector requirements
  fn delete(&mut self, s: Self::Selector) -> Result<(), Self::Error>;
  /// Delete the table from the database
  fn delete_table(&mut self) -> Result<(), Self::Error>;
}

