use super::*;

pub struct Fields<'a> {
  ident: &'a syn::Ident,
  sql_type: SqlType,
  attrs: FieldAttrs,
}

impl<'a> std::convert::TryFrom<&'a syn::Field> for Fields<'a> {
  type Error = Box<dyn std::error::Error>;
  fn try_from(f: &'a syn::Field) -> Result<Self, Self::Error> {
    let sql_type: SqlType = f.into();
    if matches!(sql_type, SqlType::Unsupported) { return Err("Type is not supported".into()); }
    Ok( 
      Fields {
        ident: f.ident.as_ref().ok_or("Field does not have an ident")?,
        sql_type,
        attrs: FieldAttrs::from_attributes(&f.attrs)?,
      } 
    )
  }
}

impl<'a> Fields<'a> {
  pub fn name(&'a self) -> String { format!("{}", self.ident) }
  pub fn ident(&'a self) -> &'a syn::Ident { self.ident }
  pub fn sql_type(&'a self) -> &'a SqlType { &self.sql_type }
  pub fn is_primary_key(&'a self) -> bool { self.attrs.is_primary_key }
  pub fn on_insert(&'a self) -> &'a Option<syn::PatPath> { &self.attrs.on_insert }
  pub fn on_update(&'a self) -> &'a Option<syn::PatPath> { &self.attrs.on_update }
}
