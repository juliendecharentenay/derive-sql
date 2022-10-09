use crate::utility;
use crate::SqlType;
use convert_case::Casing;

pub struct ImplFilter<'a> {
  pub ast: &'a syn::DeriveInput,
}

impl<'a> ImplFilter<'a> {
  pub fn generate(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let filter = self.impl_filter()?;
    let from   = self.impl_from()?;
    let q = quote::quote! {
      #filter
      #from
    };
    Ok(q)
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
   *   }
   *
   * And the associated method that takes a filter and returns
   * the SQL condition:
   *
   *   impl Filter {
   *     pub fn to_condition() -> Result<String, Box<dyn Error>>
   *
   */
  fn impl_filter(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let doc = r#"
Provides ability to nominate the filtering of results as part of the database query, aka WHERE in SQL queries
"#;

    let gt = |na: &str| syn::Ident::new(&format!("{}GreaterThan", na), proc_macro2::Span::call_site());
    let ge = |na: &str| syn::Ident::new(&format!("{}GreaterEqualThan", na), proc_macro2::Span::call_site());
    let eq = |na: &str| syn::Ident::new(&format!("{}Equal", na), proc_macro2::Span::call_site());
    let le = |na: &str| syn::Ident::new(&format!("{}LowerEqualThan", na), proc_macro2::Span::call_site());
    let lt = |na: &str| syn::Ident::new(&format!("{}LowerThan", na), proc_macro2::Span::call_site());

    let f = &utility::get_fields_named(self.ast)
       .ok_or("Unable to retrieve FieldsNamed")?
       .named
       .iter()
       .filter(|field| ! matches!(SqlType::from_type(&field.ty), SqlType::Unsupported) ) // Remove unsupported field types
       .map(|field| (&field.ty, 
                     format!("{}", field.ident.as_ref().unwrap()).to_case(convert_case::Case::Pascal), 
                     format!("{}", field.ident.as_ref().unwrap())) )
       .collect::<Vec<(&syn::Type, String, String)>>();

    
    let q: Vec<Box<dyn Fn(&str) -> syn::Ident>> = vec![Box::new(gt), Box::new(ge), Box::new(eq), Box::new(le), Box::new(lt)];
    let q = q
        .iter()
        .fold(Vec::new(), 
          |mut acc, func| {
            let mut r = f.iter().map(|(ty, name, _name_orig)| { let ident = func(name.as_str()); quote::quote! { #ident(#ty) } })
            .collect::<Vec<proc_macro2::TokenStream>>();
            acc.append(&mut r);
            acc
          }
        );
    let q_enum = quote::quote! {
      #[doc = #doc]
      pub enum Filter {
        And(Box<Filter>, Box<Filter>),
        Or(Box<Filter>, Box<Filter>),
        #( #q ),*
      }
    };

    let q: Vec<(Box<dyn Fn(&str) -> syn::Ident>, &str)> = vec![(Box::new(gt), ">"), (Box::new(ge), ">="), (Box::new(eq), "="), (Box::new(le), "<="), (Box::new(lt), "<")];
    let q = q
       .iter()
       .fold(Vec::new(),
         |mut acc, (func, op)| {
           let mut r = f.iter()
             .map(|(ty, name, name_orig)| {
                let ident = func(name.as_str());
                match SqlType::from_type(ty) {
                  SqlType::Text => quote::quote! { Filter::#ident(v) => format!("{} {} '{}'", #name_orig, #op, v) },
                  SqlType::Integer
                  | SqlType::Unsupported 
                                => quote::quote! { Filter::#ident(v) => format!("{} {} {}", #name_orig, #op, v) },
                }
             })
             .collect::<Vec<proc_macro2::TokenStream>>();
           acc.append(&mut r);
           acc
       });
    let q_impl = quote::quote! {
      impl Filter {
        pub fn to_condition(filter: &Filter) -> String {
          match filter {
            Filter::And(a, b) => format!("({} AND {})", Filter::to_condition(a), Filter::to_condition(b)),
            Filter::Or(a, b)  => format!("({} OR  {})", Filter::to_condition(a), Filter::to_condition(b)),
            #( #q ),*
          }
        }
      }
    };

    let q = quote::quote! {
      #q_enum
      #q_impl
    };
    Ok(q)
  }

  fn impl_from(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let ident = &self.ast.ident;

    let eq = |na: &str| syn::Ident::new(&format!("{}Equal", na), proc_macro2::Span::call_site());

    let q = utility::get_fields_named(self.ast)
       .ok_or("Unable to retrieve FieldsNamed")?
       .named
       .iter()
       .filter(|field| ! matches!(SqlType::from_type(&field.ty), SqlType::Unsupported) ) // Remove unsupported field types
       .map(|field| {
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
       .ok_or("Unable to construct filter")?;

    let q = quote::quote! {
      impl From<#ident> for Filter {
        fn from(item: #ident) -> Filter {
          #q
        }
      }
    };
    Ok(q)
  }
}

