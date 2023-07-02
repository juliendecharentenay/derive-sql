use super::*;

pub struct Actix<'a> {
  ast: &'a syn::DeriveInput,
  attrs: api::Attributes,
}

impl<'a> TryFrom<&'a syn::DeriveInput> for Actix<'a> {
  type Error = parse::Error;
  fn try_from(ast: &syn::DeriveInput) -> parse::Result<Actix> {
    Ok( Actix { ast, attrs: ast.try_into()? } )
  }
}

impl<'a> Actix<'a> {
  /// Generate the implementation of the API using `actix_web`
  pub fn generate(self, route: &str) -> parse::Result<proc_macro2::TokenStream> {
    let mut services: Vec<Service> = Vec::new();
    let mut functions: Vec<proc_macro2::TokenStream> = Vec::new();
    if self.attrs.list   { services.push(Service::list(&route));   functions.push(self.list()); }
    if self.attrs.post   { services.push(Service::post(&route));   functions.push(self.post()); }
    if self.attrs.get    { services.push(Service::get(&route));    functions.push(self.get()); }
    if self.attrs.patch  { services.push(Service::patch(&route));  functions.push(self.patch()?); }
    if self.attrs.delete { services.push(Service::delete(&route)); functions.push(self.delete()); }

    let config_body = services.iter()
         .map(|s| s.key.clone())
         .collect::<std::collections::BTreeSet<String>>()
         .into_iter()
         .map(|key| (key.clone(), services.iter().filter(|s| s.key.eq(&key)).map(|s| s.quote.clone()).collect::<Vec<proc_macro2::TokenStream>>() ) )
         .fold(quote::quote! {}, 
               |quote, (key, routes)| 
                 quote::quote! {
                   #quote
                   cfg.service( actix_web::web::resource(#key)#( #routes )* );
                 }
               );


    let r = quote::quote! {
      pub mod actix {
        use super::*;

        #( #functions )*

        fn to_responder<T>(r: Result<T, Box<dyn std::error::Error>>) -> impl actix_web::Responder 
        where T: serde::Serialize,
        {
          match r {
            Ok(r)  => actix_web::HttpResponse::Ok().json(r),
            Err(e) => actix_web::HttpResponse::InternalServerError().body(format!("{:#?}", e)),
          }
        }

        fn option_to_responder<T>(r: Result<Option<T>, Box<dyn std::error::Error>>) -> impl actix_web::Responder 
        where T: serde::Serialize,
        {
          match r {
            Ok(Some(r)) => actix_web::HttpResponse::Ok().json(r),
            Ok(None)    => actix_web::HttpResponse::NotFound().body("Sorry the item has not been found"),
            Err(e)      => actix_web::HttpResponse::InternalServerError().body(format!("{:#?}", e)),
          }
        }

        /// `actix_web` configuration supporting route and HTTP methods
        pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
          #config_body
        }
      }
    };
    Ok(r)
  }

  fn delete(&self) -> proc_macro2::TokenStream {
      quote::quote! {
        async fn delete(path: actix_web::web::Path<String>) -> impl actix_web::Responder {
          to_responder(handle_sql_delete(path.into_inner()).await)
        }
      }
  }

  fn patch(&self) -> parse::Result<proc_macro2::TokenStream> {
    Ok(quote::quote! {
      async fn patch(path: actix_web::web::Path<String>, body: String) -> impl actix_web::Responder {
        to_responder(handle_sql_patch(path.into_inner(), body.as_str()).await)
      }
    })
  }

  fn get(&self) -> proc_macro2::TokenStream {
      quote::quote! {
        async fn get(path: actix_web::web::Path<String>) -> impl actix_web::Responder {
          log::info!("[GET] object");
          option_to_responder(handle_sql(|sql| Ok(sql.select_one(path.into_inner().try_into()?)?) ).await)
        }
      }
  }

  fn post(&self) -> proc_macro2::TokenStream {
      quote::quote! {
        async fn post(body: String) -> impl actix_web::Responder {
          log::info!("[POST] object");
          to_responder(handle_sql(|sql| Ok(sql.insert(serde_json::from_str(body.as_str())?)?) ).await)
        }
      }
  }

  fn list(&self) -> proc_macro2::TokenStream {
      let before_sql = match &self.attrs.list_before_sql {
        Some(f) => {
          let f_ident = syn::Ident::new(f.value().as_str(), proc_macro2::Span::call_site());
          quote::quote! { let selector: sql::Selector = #f_ident(selector)?; }
        },
        None => quote::quote! {},
      };

      quote::quote! {
        async fn list(query: actix_web::web::Query<Query>) -> impl actix_web::Responder {
          log::info!("[GET] list");
          to_responder( 
            handle_sql(|sql| {
              let selector: sql::Selector = query.into_inner().try_into()?;
              #before_sql
              Ok(sql.select(selector)?)
            }).await 
          )
        }
      }
  }
}

