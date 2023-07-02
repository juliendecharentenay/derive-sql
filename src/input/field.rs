use syn::parse;

#[derive(from_attributes::DeriveFromAttributes)]
#[from_attributes(ident = "derive_sql")]
pub struct FieldAttributes {
  pub primary_key:          bool,
  on_insert:            Option<syn::LitStr>,
  on_insert_and_update: Option<syn::LitStr>,
  on_update:            Option<syn::LitStr>,
}

pub struct Field {
  pub field: syn::Field,
  pub attrs: FieldAttributes,
}

impl Field {
  pub fn span(&self)  -> proc_macro2::Span               { self.field.ident.as_ref().map(|i| i.span()).unwrap_or_else(|| proc_macro2::Span::call_site()) }
  pub fn ident(&self) -> parse::Result<&syn::Ident> { self.to_result(self.field.ident.as_ref()) }
  pub fn ident_str(&self) -> parse::Result<String>  { self.to_result(self.field.ident.as_ref().map(|i| format!("{i}"))) }
  pub fn ty(&self)    -> &syn::Type                      { &self.field.ty }
  pub fn on_insert(&self) -> Option<syn::Ident> { 
    self.attrs.on_insert.as_ref().or(self.attrs.on_insert_and_update.as_ref())
    .map(|f| syn::Ident::new(f.value().as_str(), proc_macro2::Span::call_site()) )
  }
  pub fn on_update(&self) -> Option<syn::Ident> { 
    self.attrs.on_update.as_ref().or(self.attrs.on_insert_and_update.as_ref())
    .map(|f| syn::Ident::new(f.value().as_str(), proc_macro2::Span::call_site()) )
  }

  fn to_result<T>(&self, i: Option<T>) -> parse::Result<T> {
    i.ok_or(syn::parse::Error::new(self.span(), "Unable to retrieve item"))  
  }
}

pub struct Fields { fields: Vec<Field> }
impl Into<Vec<Field>> for Fields {
  fn into(self) -> Vec<Field> { self.fields }
}

impl TryFrom<&syn::DeriveInput> for Fields {
  type Error = syn::parse::Error;

  fn try_from(ast: &syn::DeriveInput) -> syn::parse::Result<Fields> {
    match &ast.data {
      syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(named), .. }) => {
        named.named
        .iter()
        .map(|f| {
          let attrs = f.try_into()?;
          Ok(Field { field: f.clone(), attrs })
        })
        .collect::<syn::parse::Result<Vec<Field>>>()
        .map(|fields| Fields { fields } )
      },
      _ => Err(syn::parse::Error::new(ast.ident.span(), "Unsupported data. Only struct with named fields are supported.")),
    }
  }
}


