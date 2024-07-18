use super::*;

/// Generic trait to be implemented by SQL drivers (or more likely by proxy to SQL drivers) so that the 
/// functionalities provided by this crate can be leveraged
pub trait ExecuteTrait
{
  fn execute_with_params<P>(&mut self, query: &str, params: P) -> Result<()>
  where P: Params;

  fn execute(&mut self, query: &str) -> Result<()> { self.execute_with_params(query, ()) }
}

