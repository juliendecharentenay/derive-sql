use syn::parse;
use crate::attributes::api;
use crate::input::Input;

pub struct DeriveWasm<'a> {
  ast: &'a syn::DeriveInput,
  api_attrs: api::Attributes,
}

impl<'a> TryFrom<&'a syn::DeriveInput> for DeriveWasm<'a> {
  type Error = parse::Error;
  fn try_from(ast: &syn::DeriveInput) -> parse::Result<DeriveWasm> { 
    Ok( DeriveWasm { ast, api_attrs: ast.try_into()? } )
  }
}

impl<'a> DeriveWasm<'a> {
  /// Generate implementation of `wasm` client
  pub fn generate(self) -> parse::Result<proc_macro2::TokenStream> {
    if self.api_attrs.route.is_none() {
      return Err(syn::parse::Error::new(self.ast.ident.span(), r#"Attribute #[derive_sql_api(route = "..")] is missing. Please add attribute with nominating the route to be used when calling the API such as "/api/myobject"."#));
    }

    let input = Input::from(self.ast);
    let primary_key_field = input.primary_key_field()?
        .ok_or(syn::parse::Error::new(proc_macro2::Span::call_site(), "No field is marked with attribute `primary_key`"))?;
    let primary_key_ty = primary_key_field.ty();
    let class = input.class();
    let route = self.api_attrs.route.as_ref().unwrap();

    let mut quote = quote::quote! { };
    if self.api_attrs.list   { let list   = self.list()?;   quote = quote::quote! { #quote #list }; }
    if self.api_attrs.post   { let post   = self.post()?;   quote = quote::quote! { #quote #post }; }
    if self.api_attrs.get    { let get    = self.get()?;    quote = quote::quote! { #quote #get }; }
    if self.api_attrs.patch  { let patch  = self.patch()?;  quote = quote::quote! { #quote #patch }; }
    if self.api_attrs.delete { let delete = self.delete()?; quote = quote::quote! { #quote #delete }; }

    Ok(quote::quote! {
      #[wasm_bindgen::prelude::wasm_bindgen]
      impl #class {
        #quote

        async fn fetch_with_str_and_init(url: &str, request_init: web_sys::RequestInit) -> Result<web_sys::Response, wasm_bindgen::JsValue> {
          use wasm_bindgen::JsCast;
          let window = web_sys::window().ok_or("Unable to retrieve window")?;
          let request = web_sys::Request::new_with_str_and_init(url, &request_init)?;
          let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await?;
          let resp: web_sys::Response = resp_value.dyn_into()?;
          if resp.ok() {
            Ok(resp)
          } else {
            Err(format!("Request failed with status {}", resp.status_text()).into())
          }
        }

        fn url(key: Option<#primary_key_ty>) -> Result<String, wasm_bindgen::JsValue> {
          let base = web_sys::window().ok_or("Unable to retrieve window")?.location().origin()?;
          Ok(web_sys::Url::new_with_base(match key {
            Some(v) => format!("{}/{}", #route, v),
            None    => format!("{}", #route),
          }.as_str(), base.as_str())?.href())
        }

        fn url_with_search(key: Option<#primary_key_ty>, search: Vec<(&str, &str)>) -> Result<String, wasm_bindgen::JsValue> {
          let base = web_sys::window().ok_or("Unable to retrieve window")?.location().origin()?;
          let mut url = web_sys::Url::new_with_base(match key {
            Some(v) => format!("{}/{}", #route, v),
            None    => format!("{}", #route),
          }.as_str(), base.as_str())?;
          for s in search.iter() { url.search_params().append(s.0, s.1); }
          Ok(url.href())
        }

      }
    })
  }

  fn delete(&self) -> parse::Result<proc_macro2::TokenStream> {
    let input = Input::from(self.ast);
    let pk = input.primary_key_field()?
        .ok_or(syn::parse::Error::new(proc_macro2::Span::call_site(), "No field is marked with attribute `primary_key`"))?;
    let pki = pk.ident()?;
    Ok(quote::quote! {
      pub async fn delete(self) -> Result<(), wasm_bindgen::JsValue> {
        use wasm_bindgen::JsCast;
        let window = web_sys::window().ok_or("Unable to retrieve window")?;
        let mut request_init = web_sys::RequestInit::new(); request_init.method("DELETE");
        let request = web_sys::Request::new_with_str_and_init(Self::url(Some(self.#pki))?.as_str(), &request_init)?;
        let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: web_sys::Response = resp_value.dyn_into()?;
        if resp.ok() {
          Ok(())
        } else {
          Err(format!("Request failed with status {}", resp.status_text()).into())
        }
      }
    })
  }

  fn patch(&self) -> parse::Result<proc_macro2::TokenStream> {
    let input = Input::from(self.ast);
    let class = input.class();
    let pk = input.primary_key_field()?
        .ok_or(syn::parse::Error::new(proc_macro2::Span::call_site(), "No field is marked with attribute `primary_key`"))?;
    let pki = pk.ident()?;
    Ok(quote::quote! {
      pub async fn patch(self) -> Result<#class, wasm_bindgen::JsValue> {
        let body: String = serde_json::to_string(&self).map_err(|e| format!("{}", e))?;
        let body: wasm_bindgen::JsValue = serde_wasm_bindgen::to_value(&body)?;
        let mut request_init = web_sys::RequestInit::new(); request_init.body(Some(&body)).method("PATCH");
        let resp = Self::fetch_with_str_and_init(Self::url(Some(self.#pki))?.as_str(), request_init).await?;
        let js = wasm_bindgen_futures::JsFuture::from(resp.json()?).await?;
        Ok(serde_wasm_bindgen::from_value(js)?)
      }
    })
  }

  fn get(&self) -> parse::Result<proc_macro2::TokenStream> {
    let input = Input::from(self.ast);
    let pk = input.primary_key_field()?
        .ok_or(syn::parse::Error::new(proc_macro2::Span::call_site(), "No field is marked with attribute `primary_key`"))?;
    let pkty = pk.ty();
    let class = input.class();
    Ok(quote::quote! {
      pub async fn get(key: #pkty) -> Result<#class, wasm_bindgen::JsValue> {
        let mut request_init = web_sys::RequestInit::new(); request_init.method("GET");
        let resp = Self::fetch_with_str_and_init(Self::url(Some(key))?.as_str(), request_init).await?;
        let js = wasm_bindgen_futures::JsFuture::from(resp.json()?).await?;
        Ok(serde_wasm_bindgen::from_value(js)?)
      }
    })
  }

  fn post(&self) -> parse::Result<proc_macro2::TokenStream> {
    let input = Input::from(self.ast);
    let class = input.class();
    Ok(quote::quote! {
      pub async fn post(self) -> Result<#class, wasm_bindgen::JsValue> {
        use wasm_bindgen::JsCast;
        let body: String = serde_json::to_string(&self).map_err(|e| format!("{}", e))?;
        let body: wasm_bindgen::JsValue = serde_wasm_bindgen::to_value(&body)?;
        let mut request_init = web_sys::RequestInit::new(); request_init.body(Some(&body)).method("POST");
        let resp = Self::fetch_with_str_and_init(Self::url(None)?.as_str(), request_init).await?;
        let js = wasm_bindgen_futures::JsFuture::from(resp.json()?).await?;
        Ok(serde_wasm_bindgen::from_value(js)?)
      }
    })
  }

  fn list(&self) -> parse::Result<proc_macro2::TokenStream> {
    let input = Input::from(self.ast);
    let class = input.class();
    let with_lists = input.fields()?.iter()
      .map(|f| { 
        let ident_str = f.ident_str()?;
        let list_with_ident = syn::Ident::new(format!("list_with_{}", f.ident_str()?).as_str(), proc_macro2::Span::call_site());
        let list_with_ident_limit_and_offset = syn::Ident::new(format!("list_with_{}_limit_and_offset", f.ident_str()?).as_str(), proc_macro2::Span::call_site());
        Ok(quote::quote! {
          pub async fn #list_with_ident(v: String) -> Result<js_sys::Array, wasm_bindgen::JsValue> {
            let search = vec![(#ident_str, v.as_str())];
            Self::impl_list( Self::url_with_search(None, search)?.as_str(),).await
          }

          pub async fn #list_with_ident_limit_and_offset(v: String, limit: usize, offset: usize) -> Result<js_sys::Array, wasm_bindgen::JsValue> {
            let l = format!("{}", limit); let o = format!("{}", offset);
            let search = vec![(#ident_str, v.as_str()), ("limit", l.as_str()), ("offset", o.as_str())];
            Self::impl_list( Self::url_with_search(None, search)?.as_str(),).await
          }
        })
      })
      .collect::<parse::Result<Vec<proc_macro2::TokenStream>>>()?;

    Ok(quote::quote! {
      async fn impl_list(url: &str) -> Result<js_sys::Array, wasm_bindgen::JsValue> {
        let mut request_init = web_sys::RequestInit::new(); request_init.method("GET");
        let resp = Self::fetch_with_str_and_init(url, request_init).await?;
        let js = wasm_bindgen_futures::JsFuture::from(resp.json()?).await?;
        let list: Vec<#class> = serde_wasm_bindgen::from_value(js)?;
        Ok(list.into_iter().map(|i| wasm_bindgen::JsValue::from(i)).collect::<js_sys::Array>())
      }

      pub async fn list() -> Result<js_sys::Array, wasm_bindgen::JsValue> {
        Self::impl_list(Self::url(None)?.as_str()).await
      }

      pub async fn list_with_limit_and_offset(limit: usize, offset: usize) -> Result<js_sys::Array, wasm_bindgen::JsValue> {
        let l = format!("{}", limit); let o = format!("{}", offset);
        let search = vec![("limit", l.as_str()), ("offset", o.as_str())];
        Self::impl_list( Self::url_with_search(None, search)?.as_str(),).await
      }

      #( #with_lists )*

    })
  }
  
}

