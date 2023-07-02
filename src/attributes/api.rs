
#[derive(from_attributes::DeriveFromAttributes)]
#[from_attributes(ident = "derive_sql_api")]
pub struct Attributes {
  pub route:  Option<syn::LitStr>,
  pub list:   bool,
  pub list_before_sql: Option<syn::LitStr>,
  pub post:   bool,
  pub get:    bool,
  pub patch:  bool,
  pub delete: bool,
}

