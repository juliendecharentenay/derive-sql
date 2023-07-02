pub struct Service {
  pub key: String,
  pub quote: proc_macro2::TokenStream,
}
impl Service {
  pub fn post(root: &str) -> Service {
    Service {
      key: root.to_string(),
      quote: quote::quote! { .route(actix_web::web::post().to(post)) },
    }
  }

  pub fn list(root: &str) -> Service {
    Service {
      key: root.to_string(),
      quote: quote::quote! { .route(actix_web::web::get().to(list)) },
    }
  }

  pub fn get(root: &str) -> Service {
    Service {
      key: format!("{root}/{{key}}"),
      quote: quote::quote! { .route(actix_web::web::get().to(get)) },
    }
  }

  pub fn patch(root: &str) -> Service {
    Service {
      key: format!("{root}/{{key}}"),
      quote: quote::quote! { .route(actix_web::web::patch().to(patch)) },
    }
  }

  pub fn delete(root: &str) -> Service {
    Service {
      key: format!("{root}/{{key}}"),
      quote: quote::quote! { .route(actix_web::web::delete().to(delete)) },
    }
  }
}
