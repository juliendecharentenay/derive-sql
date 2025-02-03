//! Postgresql connection
use super::*;

pub struct Row {
  row: ::postgres::row::Row,
}

impl std::convert::TryFrom<::postgres::row::Row> for Row {
  type Error = Error;
  fn try_from(row: ::postgres::row::Row) -> core::result::Result<Self, Self::Error> {
    Ok( Row { row })
  }
}

impl Row {
  fn get_value_opt(&self, i: usize) -> Result<Option<traits::Value>> {
    Ok(self.row.try_get(i)?)
  }
}

impl traits::Row for Row {
  fn get_value(&self, i: usize) -> Option<Result<traits::Value>> {
    match self.get_value_opt(i) {
      Ok(Some(v)) => Some(Ok(v)),
      Ok(None) => Some(Ok(traits::Value::Null)),
      Err(e) => Some(Err(e)),
    }
  }
}

fn execute<P>(tx: &mut ::postgres::Transaction, statement: &::postgres::Statement, params: &P) -> Result<()>
where P: traits::Params,
{
    let params: Vec<traits::Param> = params.as_vec_params()?;
    let _ = match params.len() {
      0 => tx.execute(statement, &[])?,
      1 => tx.execute(statement, &[ &params[0] ] )?,
      2 => tx.execute(statement, &[ &params[0], &params[1] ] )?,
      3 => tx.execute(statement, &[ &params[0], &params[1], &params[2], ] )?,
      4 => tx.execute(statement, &[ &params[0], &params[1], &params[2], &params[3], ] )?,
      5 => tx.execute(statement, &[ &params[0], &params[1], &params[2], &params[3], &params[4], ] )?,
      6 => tx.execute(statement, &[ &params[0], &params[1], &params[2], &params[3], &params[4], &params[5], ] )?,
      7 => tx.execute(statement, &[ &params[0], &params[1], &params[2], &params[3], &params[4], &params[5], &params[6], ] )?,
      8 => tx.execute(statement, &[ &params[0], &params[1], &params[2], &params[3], &params[4], &params[5], &params[6], &params[7], ] )?,
      9 => tx.execute(statement, &[ &params[0], &params[1], &params[2], &params[3], &params[4], &params[5], &params[6], &params[7], &params[8], ] )?,
     10 => tx.execute(statement, &[ &params[0], &params[1], &params[2], &params[3], &params[4], &params[5], &params[6], &params[7], &params[8], &params[9], ] )?,
     11 => tx.execute(statement, &[ &params[0], &params[1], &params[2], &params[3], &params[4], &params[5], &params[6], &params[7], &params[8], &params[9], &params[10], ] )?,
     12 => tx.execute(statement, &[ &params[0], &params[1], &params[2], &params[3], &params[4], &params[5], &params[6], &params[7], &params[8], &params[9], &params[10], &params[11], ] )?,
     13 => tx.execute(statement, &[ &params[0], &params[1], &params[2], &params[3], &params[4], &params[5], &params[6], &params[7], &params[8], &params[9], &params[10], &params[11], &params[12], ] )?,
     14 => tx.execute(statement, &[ &params[0], &params[1], &params[2], &params[3], &params[4], &params[5], &params[6], &params[7], &params[8], &params[9], &params[10], &params[11], &params[12], &params[13], ] )?,
     15 => tx.execute(statement, &[ &params[0], &params[1], &params[2], &params[3], &params[4], &params[5], &params[6], &params[7], &params[8], &params[9], &params[10], &params[11], &params[12], &params[13], &params[14], ] )?,
     16 => tx.execute(statement, &[ &params[0], &params[1], &params[2], &params[3], &params[4], &params[5], &params[6], &params[7], &params[8], &params[9], &params[10], &params[11], &params[12], &params[13], &params[14], &params[15], ] )?,
     17 => tx.execute(statement, &[ &params[0], &params[1], &params[2], &params[3], &params[4], &params[5], &params[6], &params[7], &params[8], &params[9], &params[10], &params[11], &params[12], &params[13], &params[14], &params[15], &params[16], ] )?,
      // _ => { self.conn.execute(query, params.iter().collect::<Vec<&traits::Param>>().as_slice())?; },
      _ => { return Err(Error::MaximumNumberOfParametersExceeded(17, params.len())); },
    };
    Ok(())
}

impl traits::Connection<Row> for ::postgres::Client 
{
  fn flavor(&self) -> traits::Flavor { traits::Flavor::PostgreSQL }

  fn execute_with_params<S, P>(&mut self, query: S, params: &P) -> Result<()>
  where S: std::convert::AsRef<str>,
        P: traits::Params,
  {
    let mut tx = self.transaction()?;
    let statement = tx.prepare(query.as_ref())?;
    execute(&mut tx, &statement, params)?;
    tx.commit()?;
    Ok(())
  }

  fn execute_with_params_iterator<'a, S, I, P>(&mut self, query: S, params_iter: I) -> Result<()>
  where S: std::convert::AsRef<str>,
        P: traits::Params + 'a,
        I: core::iter::IntoIterator<Item = &'a P>
  {
    let mut tx = self.transaction()?;
    let statement = tx.prepare(query.as_ref())?;
    for params in params_iter { execute(&mut tx, &statement, params)?; }
    tx.commit()?;
    Ok(())
  }

  fn query<S>(&mut self, query: S) -> Result<Vec<Row>>
  where S: std::convert::AsRef<str>
  {
    ::log::info!("Running query");
    let r = self.query(query.as_ref(), &[])?.into_iter()
    .map(|row| {
      ::log::info!("Mapping row {row:?}");
      Ok(row.try_into()?) 
    })
    .collect::<Result<Vec<Row>>>()?;
    ::log::info!("Return query result");
    Ok(r)
  }
}

