#[derive(from_attributes::DeriveFromAttributes)]
#[from_attributes(ident = "derive_sql")]
pub struct Attributes {
  pub rusqlite_connection: Option<syn::LitStr>,
  pub provide_api:         bool,
  pub provide_api_actix:   bool,
  pub provide_api_lambda:  bool,
  pub provide_wasm_client: bool,
  pub no_sql:              bool,
} 

