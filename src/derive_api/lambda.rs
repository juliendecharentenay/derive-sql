use super::*;

pub struct Lambda<'a> {
  ast: &'a syn::DeriveInput,
  attrs: api::Attributes,
}

impl<'a> TryFrom<&'a syn::DeriveInput> for Lambda<'a> {
  type Error = parse::Error;
  fn try_from(ast: &syn::DeriveInput) -> parse::Result<Lambda> {
    Ok( Lambda { ast, attrs: ast.try_into()? } )
  }
}

impl<'a> Lambda<'a> {
  /// Generate the implementation of the API using `lambda_http`
  pub fn generate(self, route: &str) -> parse::Result<proc_macro2::TokenStream> {
    let mut services: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut functions: Vec<proc_macro2::TokenStream> = Vec::new();
    if self.attrs.list   { services.push(Service::list());   functions.push(self.list()?); }
    if self.attrs.post   { services.push(Service::post());   functions.push(self.post()?); }
    if self.attrs.get    { services.push(Service::get());    functions.push(self.get()?); }
    if self.attrs.patch  { services.push(Service::patch());  functions.push(self.patch()?); }
    if self.attrs.delete { services.push(Service::delete()); functions.push(self.delete()?); }

    let handler = quote::quote! {
        async fn handler(event: &lambda_http::Request) -> Result<Option<lambda_http::Response<String>>, Box<dyn std::error::Error>> {
          use lambda_http::RequestExt;
          if let lambda_http::request::RequestContext::ApiGatewayV1(context) = event.request_context() {
            let path = context.resource_path.as_ref().ok_or("Unable to retrieve resource path")?;
            if path.starts_with(#route) {
              let params = event.path_parameters();
              let body = match event.body() {
                lambda_http::Body::Text(body) => Some(body),
                _ => None,
              };
              match context.http_method {
                #( #services ),*
                _ => Err("Request is not supported".into()),
              }
            } else {
              Ok(None)
            }
            
          } else {
            Err("Request is not of type ApiGatewayV1".into())
          }
        }
    };

    let r = quote::quote! {
      pub mod lambda {
        use super::*;
        #handler
        #( #functions )*

        /// Handle HTTP request
        pub async fn handle(event: &lambda_http::Request) -> Result<Option<lambda_http::Response<String>>, lambda_http::Error> {
          match handler(event).await {
            Ok(r) => Ok(r),
            Err(e) => {
              log::error!("Error: {:#?}", e);
              Ok(Some(lambda_http::Response::builder()
              .status(lambda_http::http::StatusCode::INTERNAL_SERVER_ERROR)
              .body(e.to_string())?))
            }
          }
        }

        // Convert the result of an operation to a lambda_http::Response
        fn to_response<T>(r: Result<T, Box<dyn std::error::Error>>) -> Result<Option<lambda_http::Response<String>>, Box<dyn std::error::Error>>
        where T: serde::Serialize
        {
          match r {
            Ok(r)  => Ok(Some(lambda_http::Response::builder().status(lambda_http::http::StatusCode::OK).body(serde_json::to_string(&r)?)?)),
            Err(e) => Ok(Some(lambda_http::Response::builder().status(lambda_http::http::StatusCode::INTERNAL_SERVER_ERROR).body(e.to_string())?)),
          }
        }

        // Convert the result of an operation that returns an Option to a lambda_http::Response
        fn option_to_response<T>(r: Result<Option<T>, Box<dyn std::error::Error>>) -> Result<Option<lambda_http::Response<String>>, Box<dyn std::error::Error>>
        where T: serde::Serialize
        {
          match r {
            Ok(Some(r)) => Ok(Some(lambda_http::Response::builder().status(lambda_http::http::StatusCode::OK).body(serde_json::to_string(&r)?)?)),
            Ok(None)    => Ok(Some(lambda_http::Response::builder().status(lambda_http::http::StatusCode::NOT_FOUND).body("Sorry the item has not been found".to_string())?)),
            Err(e)      => Ok(Some(lambda_http::Response::builder().status(lambda_http::http::StatusCode::INTERNAL_SERVER_ERROR).body(e.to_string())?)),
          }
        }
      }
    };
    Ok(r)
  }

  fn list(&self) -> parse::Result<proc_macro2::TokenStream> {
    let before_sql = match &self.attrs.list_before_sql {
      Some(f) => {
        let f_ident = syn::Ident::new(f.value().as_str(), proc_macro2::Span::call_site());
        quote::quote! { let selector: sql::Selector = #f_ident(selector)?; }
      },
      None => quote::quote! {},
    };

    Ok(quote::quote! {
      async fn list(query: Query) -> Result<Option<lambda_http::Response<String>>, Box<dyn std::error::Error>> {
        log::info!("[GET] list of object");
        to_response(handle_sql(|sql| {
          let selector: sql::Selector = query.try_into()?;
          #before_sql
          Ok(sql.select(selector)?)
        }).await)
      }
    })
  }

  fn get(&self) -> parse::Result<proc_macro2::TokenStream> {
    Ok(quote::quote! {
      async fn get(key: String) -> Result<Option<lambda_http::Response<String>>, Box<dyn std::error::Error>> {
        log::info!("[GET] object");
        option_to_response(handle_sql(|sql| Ok(sql.select_one(key.try_into()?)?)).await)
      }
    })
  }

  fn post(&self) -> parse::Result<proc_macro2::TokenStream> {
    Ok(quote::quote! {
      async fn post(body: &str) -> Result<Option<lambda_http::Response<String>>, Box<dyn std::error::Error>> {
        log::info!("[POST] object");
        to_response(handle_sql(|sql| Ok(sql.insert(serde_json::from_str(body)?)?) ).await)
      }
    })
  }

  fn patch(&self) -> parse::Result<proc_macro2::TokenStream> {
    Ok(quote::quote! {
      async fn patch(key: String, body: &str) -> Result<Option<lambda_http::Response<String>>, Box<dyn std::error::Error>> {
        to_response(handle_sql_patch(key, body).await)
      }
    })
  }

  fn delete(&self) -> parse::Result<proc_macro2::TokenStream> {
    Ok(quote::quote! {
      async fn delete(key: String) -> Result<Option<lambda_http::Response<String>>, Box<dyn std::error::Error>> {
        to_response(handle_sql_delete(key).await)
      }
    })
  }
}

struct Service { }
impl Service {
  pub fn get() -> proc_macro2::TokenStream {
    quote::quote! {
      lambda_http::http::method::Method::GET if params.first("key").is_some() => { 
        get(urlencoding::decode(params.first("key").unwrap())?.to_string()).await
      }
    }
  }
  pub fn list() -> proc_macro2::TokenStream {
    quote::quote! {
      lambda_http::http::method::Method::GET => { 
        list(serde_qs::from_str(event.query_string_parameters().to_query_string().as_str())?).await
      }
    }
  }
  pub fn post() -> proc_macro2::TokenStream {
    quote::quote! {
      lambda_http::http::method::Method::POST if body.is_some() => { 
        post(body.unwrap().as_str()).await
      }
    }
  }
  pub fn delete() -> proc_macro2::TokenStream {
    quote::quote! {
      lambda_http::http::method::Method::DELETE if params.first("key").is_some() => { 
        delete(urlencoding::decode(params.first("key").unwrap())?.to_string()).await
      }
    }
  }
  pub fn patch() -> proc_macro2::TokenStream {
    quote::quote! {
      lambda_http::http::method::Method::PATCH if params.first("key").is_some() && body.is_some() => { 
        patch(urlencoding::decode(params.first("key").unwrap())?.to_string(), body.unwrap().as_str()).await
      }
    }
  }
}
