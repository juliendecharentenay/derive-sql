use convert_case::Casing;

use super::*;

pub struct Selector<'a> {
  ast: &'a syn::DeriveInput,
}

impl<'a> Selector<'a> {
  pub fn new(ast: &'a syn::DeriveInput) -> Selector { Selector { ast } }
  pub fn generate(self) -> syn::parse::Result<proc_macro2::TokenStream> {
    let filter = filter::Filter::new(self.ast).generate()?;

    let from = self.impl_from()?;
    let to_statement = self.impl_to_statement()?;

    let doc = "Struct to collate query information regarding filtering, pagination and convert this information to SQL statement";
    Ok(quote::quote! { 
      #filter 

      #[doc = #doc]
      pub struct Selector {
        filter: Option<Filter>,
        limit: Option<usize>,
        offset: Option<usize>,
      }

      impl Selector {
        #to_statement
        pub fn with_limit(mut self, v: usize) -> Selector  { self.limit = Some(v); self }
        pub fn with_offset(mut self, v: usize) -> Selector { self.offset = Some(v); self }
        pub fn filter(&self) -> &Option<Filter>       { &self.filter }
        pub fn with_filter(mut self, v: Filter) -> Selector { self.filter = Some(v); self }
      }
      #from
    })
  }

  fn impl_from(&'a self) -> syn::parse::Result<proc_macro2::TokenStream> {
    let mut quote = quote::quote! {
      impl TryFrom<()> for Selector {
        type Error = Box<dyn std::error::Error>;
        fn try_from(v: ()) -> Result<Self, Self::Error> {
          Ok(Selector { filter: None, limit: None, offset: None, })
        }
      }

      impl TryFrom<Filter> for Selector {
        type Error = Box<dyn std::error::Error>;
        fn try_from(f: Filter) -> Result<Self, Self::Error> {
          Ok(Selector { filter: Some(f), limit: None, offset: None, })
        }
      }
    };
    if let Some(primary_field) = Input::from(self.ast).fields()? // Into::<Vec<field::Field>>::into(field::Fields::try_from(self.ast)?)
      .iter().find(|f| f.attrs.primary_key) {
      let c = primary_field.ty();
      let i = primary_field.ident()?;
      let filter = syn::Ident::new(&format!("{}Equal", primary_field.ident_str()?.to_case(convert_case::Case::Pascal)), proc_macro2::Span::call_site());
      quote = quote::quote! {
        #quote

        impl TryFrom<#c> for Selector {
          type Error = Box<dyn std::error::Error>;
          fn try_from(#i: #c) -> Result<Self, Self::Error> {
            Ok(Selector { filter: Some(Filter::#filter(#i)), limit: None, offset: None, })
          }
        }
      }
    }
    Ok(quote)
  }

  fn impl_to_statement(&'a self) -> syn::parse::Result<proc_macro2::TokenStream> {
    Ok(quote::quote! {
      pub fn to_statement(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut statements = Vec::new();
        if let Some(filter) = &self.filter { statements.push(format!("WHERE {}", filter.to_condition())); }
        if let Some(limit)  = &self.limit  { statements.push(format!("LIMIT {}", limit)); }
        if let Some(offset) = &self.offset { statements.push(format!("OFFSET {}", offset)); }
        Ok(statements.join(" "))
      }
    })
  }
}

