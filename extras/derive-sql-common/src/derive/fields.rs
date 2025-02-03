use super::*;

use attribute_derive::{Attribute};

#[derive(Attribute)]
#[attribute(ident = derive_sqlite)]
struct FieldAttrs {
  #[attribute(default = false)]
  is_primary_key: bool,
  #[attribute(default = false)]
  is_unique: bool,
  on_insert: Option<syn::PatPath>,
  on_update: Option<syn::PatPath>,
}

pub struct Fields<'a> {
  ident: &'a syn::Ident,
  sql_type: SqlType,
  attrs: FieldAttrs,
  raw_type: String,
}

impl<'a> std::convert::TryFrom<&'a syn::Field> for Fields<'a> {
  type Error = Box<dyn std::error::Error>;
  fn try_from(f: &'a syn::Field) -> Result<Self, Self::Error> {
    let sql_type: SqlType = f.into();
    let raw_type: String = extract_type(&f.ty).ok_or(format!("Unable to retrieve raw type for {:#?}", &f.ty))?;
    if matches!(sql_type, SqlType::Unsupported) { return Err("Type is not supported".into()); }
    Ok( 
      Fields {
        ident: f.ident.as_ref().ok_or("Field does not have an ident")?,
        sql_type,
        attrs: FieldAttrs::from_attributes(&f.attrs)?,
        raw_type,
      } 
    )
  }
}

impl<'a> Fields<'a> {
  pub fn name(&'a self) -> String { format!("{}", self.ident) }
  pub fn ident(&'a self) -> &'a syn::Ident { self.ident }
  pub fn raw_type(&'a self) -> &'a str { self.raw_type.as_str() }
  pub fn sql_type(&'a self) -> &'a SqlType { &self.sql_type }
  pub fn is_primary_key(&'a self) -> bool { self.attrs.is_primary_key }
  pub fn is_unique(&self) -> bool { self.attrs.is_unique }
  pub fn on_insert(&'a self) -> &'a Option<syn::PatPath> { &self.attrs.on_insert }
  pub fn on_update(&'a self) -> &'a Option<syn::PatPath> { &self.attrs.on_update }
  pub fn as_pub_static_member(&'a self) -> proc_macro2::TokenStream {
    let key: syn::Ident = syn::Ident::new(self.name().to_ascii_uppercase().as_str(), self.ident.span()); 
    let value = self.name();
    quote::quote! {
      pub const #key : &'static str = #value;
    }
  }
}

/// Retrieve the final type as a string, such "std::option::Option<f32>" gives "f32"
fn extract_type(ty: &syn::Type) -> Option<String> {
  match ty {
    syn::Type::Path(syn::TypePath { path: syn::Path { segments, .. } , .. }) => {
      match segments.last() {
        Some(syn::PathSegment { ident,
            arguments: syn::PathArguments::AngleBracketed( syn::AngleBracketedGenericArguments { args, ..  } )
        }) if ident == "Option" => {
          match args.last() {
            Some(syn::GenericArgument::Type(syn::Type::Path(syn::TypePath { path, .. }))) => path.get_ident().map(|i| i.to_string()),
            _ => None,
          }
        },
        Some(syn::PathSegment { ident, ..}) => Some(ident.to_string()),
        _ => None,
      }
    },
    // syn::Type::Path(syn::TypePath { path, .. }) => path.get_ident().map(|i| i.to_string()),
    _ => None,
  }
}

