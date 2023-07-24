// use crate::{SqlType, derive_sql, input::Input};
use convert_case::Casing;

use super::*;

pub struct Filter<'a> {
  ast: &'a syn::DeriveInput,
}

impl<'a> Filter<'a> {
  pub fn new(ast: &'a syn::DeriveInput) -> Filter { Filter { ast } }
  pub fn generate(self) -> syn::parse::Result<proc_macro2::TokenStream> {
    let filter = self.impl_filter()?;
    // let from   = self.impl_from()?;
    Ok(quote::quote! {
      #filter
    })
  }

  /*
   * output enum to be used for query filtering (ie where):
   *   enum Filter {
   *     And(Box<Filter>, Box<Filter>),
   *     Or(Box<Filter>, Box<Filter>),
   *     FieldGreaterThan(Field),
   *     FieldGreaterEqualThan(Field),
   *     FieldEqual(Field),
   *     FieldLowerEqualThan(Field),
   *     FieldLowerThan(Field),
   *     None,
   *     All,
   *   }
   *
   * And the associated method that takes a filter and returns
   * the SQL condition:
   *
   *   impl Filter {
   *     pub fn to_condition() -> syn::parse::Result<String, Box<dyn Error>>
   *
   */
  fn impl_filter(&'a self) -> syn::parse::Result<proc_macro2::TokenStream> {
    let doc = r#"
Provides ability to nominate the filtering of results as part of the database query, aka WHERE in SQL queries
"#;

    let gt = |na: &str| syn::Ident::new(&format!("{}GreaterThan", na), proc_macro2::Span::call_site());
    let ge = |na: &str| syn::Ident::new(&format!("{}GreaterEqualThan", na), proc_macro2::Span::call_site());
    let eq = |na: &str| syn::Ident::new(&format!("{}Equal", na), proc_macro2::Span::call_site());
    let le = |na: &str| syn::Ident::new(&format!("{}LowerEqualThan", na), proc_macro2::Span::call_site());
    let lt = |na: &str| syn::Ident::new(&format!("{}LowerThan", na), proc_macro2::Span::call_site());

    let fields = Input::from(self.ast).fields()?;
    let f = fields.iter()
       .filter(|f| ! matches!(SqlType::from(*f), SqlType::Unsupported) ) // Remove unsupported field types
       .map(|f| Ok((f.ty(), f.ident_str()?.to_case(convert_case::Case::Pascal), f.ident_str()?, SqlType::from(f))) )
       .collect::<syn::parse::Result<Vec<(&syn::Type, String, String, SqlType)>>>()?;
    
    let q: Vec<Box<dyn Fn(&str) -> syn::Ident>> = vec![Box::new(gt), Box::new(ge), Box::new(eq), Box::new(le), Box::new(lt)];
    let q = q.iter()
        .map(|func| f.iter().map(|(ty, name, _name_orig, _sql_type)| { let ident = func(name.as_str()); quote::quote! { #ident(#ty) } })
                    .collect::<Vec<proc_macro2::TokenStream>>()
        ).flatten().collect::<Vec<proc_macro2::TokenStream>>();

    let q_enum = quote::quote! {
      #[doc = #doc]
      #[derive(Clone)]
      pub enum Filter {
        And(Box<Filter>, Box<Filter>),
        Or(Box<Filter>, Box<Filter>),
        None,
        All,
        #( #q ),*
      }
    };

    let q: Vec<(Box<dyn Fn(&str) -> syn::Ident>, &str)> = vec![(Box::new(gt), ">"), (Box::new(ge), ">="), (Box::new(eq), "="), (Box::new(le), "<="), (Box::new(lt), "<")];
    let q = q.iter()
       .map(|(func, op)| 
         f.iter().map(|(_ty, name, name_orig, sql_type)| {
           let ident = func(name.as_str());
           match sql_type {
             SqlType::Text 
             | SqlType::DateTime => quote::quote! { Filter::#ident(v) => format!("{} {} '{}'", #name_orig, #op, v) },
             SqlType::Integer
             | SqlType::Boolean
             | SqlType::Float
             | SqlType::Unsupported => quote::quote! { Filter::#ident(v) => format!("{} {} {}", #name_orig, #op, v) },
           }
         }).collect::<Vec<proc_macro2::TokenStream>>()
       ).flatten().collect::<Vec<proc_macro2::TokenStream>>();
       
    let q_impl = quote::quote! {
      impl Filter {
        pub fn to_condition(&self) -> String {
          match self {
            Filter::And(a, b) => format!("({} AND {})", Filter::to_condition(a), Filter::to_condition(b)),
            Filter::Or(a, b)  => format!("({} OR  {})", Filter::to_condition(a), Filter::to_condition(b)),
            Filter::None      => "1=0".to_string(),
            Filter::All       => "1=1".to_string(),
            #( #q ),*
          }
        }
      }
    };

    Ok(quote::quote! {
      #q_enum
      #q_impl
    })
  }

/*
  fn impl_from(&'a self) -> syn::parse::Result<proc_macro2::TokenStream> {
    let eq = |na: &str| syn::Ident::new(&format!("{}Equal", na), proc_macro2::Span::call_site());

    let q = self.parent.fields()?.iter()
       .filter(|f| ! matches!(f.sql_type, SqlType::Unsupported) ) // Remove unsupported field types
       .map(|f| {
         let ident = field.ident.as_ref().unwrap();
         let filter_ident = eq(format!("{}", ident).to_case(convert_case::Case::Pascal).as_str());
         quote::quote! {
           Filter::#filter_ident(item.#ident)
         }
       })
       .fold(None, |acc, q| {
         match acc {
           None => Some(q),
           Some(o) => Some(quote::quote! { Filter::And( Box::new(#o), Box::new(#q) ) }),
         }
       })
       .ok_or(syn::parse::Error::new(proc_macro2::Span::call_site(), "Unable to construct filter"))?;

    let q = quote::quote! {
      impl From<#ident> for Filter {
        fn from(item: #ident) -> Filter {
          #q
        }
      }
    };
    Ok(q)
  }
  */
}

