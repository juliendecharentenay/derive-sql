use syn::parse;
use crate::input::Input;
use crate::attributes::{sql, api};

mod service; use service::Service;
mod actix; mod lambda;

pub struct DeriveApi<'a> {
  ast: &'a syn::DeriveInput,
  attrs: api::Attributes,
  attrs_sql: sql::Attributes,
}

impl<'a> TryFrom<&'a syn::DeriveInput> for DeriveApi<'a> {
  type Error = parse::Error;
  fn try_from(ast: &syn::DeriveInput) -> parse::Result<DeriveApi> { 
    Ok( DeriveApi { ast, attrs: ast.try_into()?, attrs_sql: ast.try_into()? } )
  }
}

impl<'a> DeriveApi<'a> {
  /// Generate the implementation of the API
  pub fn generate(self) -> parse::Result<proc_macro2::TokenStream> {
    if self.attrs.route.is_none() {
      return Err(syn::parse::Error::new(self.ast.ident.span(), r#"Attribute #[derive_sql_api(route = "..")] is missing. Please add attribute with nominating the route to be used when calling the API such as "/api/myobject"."#));
    }
    let route = self.attrs.route.as_ref().unwrap().value();

    if self.attrs_sql.provide_api_actix == false && self.attrs_sql.provide_api_lambda == false {
      return Err(syn::parse::Error::new(self.ast.ident.span(), r#"Attribute #[derive_sql_api(provide_api_actix)] or #[derive_sql_api(provide_api_lambda)] is required from attribute #[derive_sql_api(provide_api)] is activated."#));
    }

    let input     = Input::from(self.ast);
    let class_sql = input.class_sql();
    let query     = self.query()?;
    let actix = if self.attrs_sql.provide_api_actix {
      actix::Actix::try_from(self.ast)?.generate(route.as_str())?
    } else {
      quote::quote! { }
    };
    let lambda = if self.attrs_sql.provide_api_lambda {
      lambda::Lambda::try_from(self.ast)?.generate(route.as_str())?
    } else {
      quote::quote! { }
    };

    let mut functions: Vec<proc_macro2::TokenStream> = Vec::new();
    if self.attrs.delete { functions.push(self.handle_sql_delete()?); }
    if self.attrs.patch  { functions.push(self.handle_sql_patch()?); }

    let r = quote::quote! {
      pub mod api {
        use super::*;
        #query
        #( #functions )*
        async fn handle_sql<F, R>(f: F) -> Result<R, Box<dyn std::error::Error>> 
        where F: FnOnce(#class_sql) -> Result<R, Box<dyn std::error::Error>>
        {
          let sql = #class_sql::from_database()?;
          sql.create_table()?;
          Ok(f(sql)?)
        }
        #actix
        #lambda
      }
    };
    Ok(r)
  }

  fn handle_sql_delete(&self) -> parse::Result<proc_macro2::TokenStream> {
    Ok(quote::quote! {
      async fn handle_sql_delete(key: String) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("[DELETE] object {}", key);
        handle_sql(|sql| Ok(sql.delete(key.try_into()?)?) ).await
      }
    })
  }

  fn handle_sql_patch(&self) -> parse::Result<proc_macro2::TokenStream> {
    let input = Input::from(self.ast);
    let class = input.class();
    let (k_ty, k_ident) = input.fields()?
      .iter()
      .find(|f| f.attrs.primary_key)
      .ok_or(syn::parse::Error::new(proc_macro2::Span::call_site(), "primary_key is required"))
      .and_then(|f| Ok((f.ty().clone(), f.ident()?.clone())) )?;
      
    Ok(quote::quote! {
      async fn handle_sql_patch(key: String, body: &str) -> Result<#class, Box<dyn std::error::Error>> {
        log::info!("[PATCH] object {}", key);
        handle_sql(|sql| {
          let k: #k_ty = key.try_into()?;
          let mut o: #class = serde_json::from_str(body)?;
          o.#k_ident   = k.clone();
          Ok(sql.update(k.try_into()?, o)?)
        }).await
      }
    })
  }

  fn query(&self) -> parse::Result<proc_macro2::TokenStream> {
    use convert_case::Casing;
    let input = Input::from(self.ast);
    let fields = input.fields()?.iter()
        .map(|f| { let ident = f.ident()?; let ty = f.ty(); Ok(quote::quote! { #ident: Option<#ty> }) })
        .collect::<parse::Result<Vec<proc_macro2::TokenStream>>>()?;

    let filter_statements = input.fields()?.iter()
        .map(|f| { 
          let ident = f.ident()?; 
          let filter_ident_str = format!("{}Equal", f.ident_str()?.to_case(convert_case::Case::Pascal));
          let filter_ident = syn::Ident::new(filter_ident_str.as_str(), proc_macro2::Span::call_site());
          Ok(quote::quote! {
            let filter = match self.#ident {
              Some(v) => {
                match filter {
                  Some(f) => Some(sql::Filter::And(Box::new(f), Box::new(sql::Filter::#filter_ident(v)))),
                  None => Some(sql::Filter::#filter_ident(v)),
                }
              },
              None => filter,
            };
          })
        })
        .collect::<parse::Result<Vec<proc_macro2::TokenStream>>>()?;

    Ok(quote::quote! {
      #[derive(serde::Deserialize, Debug)]
      struct Query {
        #( #fields ),* ,
        limit: Option<usize>,
        offset: Option<usize>,
      }

      impl TryInto<sql::Selector> for Query {
        type Error = Box<dyn std::error::Error>;
        fn try_into(self) -> Result<sql::Selector, Self::Error> {
          let filter: Option<sql::Filter> = None;
          #( #filter_statements )*
          let selector: sql::Selector = filter.map(|f| f.try_into()).unwrap_or_else(|| ().try_into())?;
          let selector = match self.limit {
            Some(v) => selector.with_limit(v),
            None => selector,
          };
          let selector = match self.offset {
            Some(v) => selector.with_offset(v),
            None => selector,
          };
          Ok(selector)
        }
      }
    })
  }
}
