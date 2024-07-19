use super::*;

enum Filter {
  FilterNone(filter::Filterable<filter::generic::Filter<String>>),
  FilterStr(filter::Filterable<filter::generic::Filter<String>>),
  FilterU32(filter::Filterable<filter::generic::Filter<u32>>),
}

impl Filter {
  fn and(self, next: Box<dyn Selectable>) -> Filter {
    match self {
      Filter::FilterNone(v) => Filter::FilterNone(v.and(next)),
      Filter::FilterStr(v) => Filter::FilterStr(v.and(next)),
      Filter::FilterU32(v) => Filter::FilterU32(v.and(next)),
    }
  }

  fn filter(&self) -> Option<String> {
    match self {
      Filter::FilterNone(v) => filter::Filterable::filter(v),
      Filter::FilterStr(v) => filter::Filterable::filter(v),
      Filter::FilterU32(v) => filter::Filterable::filter(v),
    }
  }

  fn next(&self) -> &Option<Box<dyn Selectable>> {
    match self {
      Filter::FilterNone(v) => v.next(),
      Filter::FilterStr(v) => v.next(),
      Filter::FilterU32(v) => v.next(),
    }
  }
}

pub struct SimpleFilter {
  filter: Filter,
}

impl SimpleFilter {
  pub fn and(mut self, next: Box<dyn Selectable>) -> SimpleFilter { self.filter = self.filter.and(next); self }
}

impl Selectable for SimpleFilter {
  fn filter(&self) -> Option<String> { self.filter.filter() }
  fn limit(&self) -> Option<usize> { self.filter.next().as_ref().and_then(|n| n.limit()) }
  fn offset(&self) -> Option<usize> { self.filter.next().as_ref().and_then(|n| n.offset()) }
  fn order_by(&self) -> Option<String> { self.filter.next().as_ref().and_then(|n| n.order_by()) }
}

impl std::convert::TryFrom<()> for SimpleFilter {
  type Error = Box<dyn std::error::Error>;
  fn try_from(_: ()) -> std::result::Result<Self, Self::Error> {
    Ok(SimpleFilter { filter: Filter::FilterNone(
      std::convert::From::<filter::Filterable<filter::generic::Filter<String>>>::from(().into())
    ) }) // None, next: None })
  }
}

impl std::convert::TryFrom<(String, String)> for SimpleFilter {
  type Error = Box<dyn std::error::Error>;
  fn try_from((key, value): (String, String)) -> std::result::Result<Self, Self::Error> {
    let filter: filter::generic::Filter<String> = (key, filter::Operator::Equal, value::Value::<String>::from(value)).into();
    let filter: filter::Filterable<_> = filter.into();
    Ok(SimpleFilter { filter: Filter::FilterStr(filter) })
  }
}

impl std::convert::TryFrom<(&str, &str)> for SimpleFilter {
  type Error = Box<dyn std::error::Error>;
  fn try_from((key, value): (&str, &str)) -> std::result::Result<Self, Self::Error> {
    let filter: filter::generic::Filter<String> = (key, filter::Operator::Equal, value::Value::<String>::from(value)).into();
    let filter: filter::Filterable<_> = filter.into();
    Ok(SimpleFilter { filter: Filter::FilterStr(filter) })
  }
}

impl std::convert::TryFrom<(&str, u32)> for SimpleFilter {
  type Error = Box<dyn std::error::Error>;
  fn try_from((key, value): (&str, u32)) -> std::result::Result<Self, Self::Error> {
    let filter: filter::generic::Filter<_> = (key, filter::Operator::Equal, value::Value::<u32>::from(value)).into();
    let filter: filter::Filterable<_> = filter.into();
    Ok(SimpleFilter { filter: Filter::FilterU32(filter) })
    // Ok(SimpleFilter { filter: Filter::FilterU32((key, filter::Operator::Equal, value::Value::<u32>::from(value)).into()) }) // Some((key.to_string(), Value::ValueU32(value))), next: None, })
  }
}
