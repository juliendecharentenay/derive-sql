use syn::parse;

mod field; pub use field::Field;

/// Wrapper around `proc_macro` struct input
pub struct Input<'a> {
  ast: &'a syn::DeriveInput,
}

impl<'a> From<&'a syn::DeriveInput> for Input<'a> {
  /// Convert `proc_macro` input (`DeriveInput`) into the wrapper `Input` struct
  fn from(ast: &syn::DeriveInput) -> Input { Input { ast } }
}

impl<'a> Input<'a> {

  /// Retrieve struct visibility
  pub fn visibility(&self) -> &syn::Visibility { &self.ast.vis }

  /// Retrieve struct name (identifier)
  pub fn class(&self) -> &syn::Ident           { &self.ast.ident }

  /// Retrieve identifier associated
  pub fn class_sql(&self) -> syn::Ident        { syn::Ident::new(format!("{}Sql", self.ast.ident).as_str(), self.ast.ident.span()) }

  /// Retrieve the list of fields contained in the struct
  pub fn fields(&self) -> parse::Result<Vec<field::Field>> {
    Ok(Into::<Vec<Field>>::into(field::Fields::try_from(self.ast)?))
  }

  /// Retrieve the list of identifier container in the struct
  pub fn idents(&self) -> parse::Result<Vec<syn::Ident>> {
    Ok(self.fields()?
    .iter()
    .map(|field| field.field.ident.as_ref().ok_or(syn::parse::Error::new(proc_macro2::Span::call_site(), "Error retrieving ident")))
    .collect::<parse::Result<Vec<&syn::Ident>>>()?
    .into_iter().cloned().collect())
  }

  /// Retrieve the field annotated with `primary_key` if available
  pub fn primary_key_field(&self) -> parse::Result<Option<field::Field>> {
    Ok(self.fields()?.into_iter().find(|f| f.attrs.primary_key))
  }
}
