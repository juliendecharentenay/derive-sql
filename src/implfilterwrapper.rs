

pub struct ImplFilterWrapper<'a> {
  pub ast: &'a syn::DeriveInput,
  pub struct_ident: syn::Ident,
  pub builder_ident: syn::Ident,
}

impl<'a> ImplFilterWrapper<'a> {
  pub fn generate(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let struct_     = self.impl_struct()?;
    let from_filter = self.impl_from_filter()?;
    let from_struct = self.impl_from_struct()?;
    let builder     = self.impl_builder()?;
    let q = quote::quote! {
      #struct_
      #from_filter
      #from_struct
      #builder
    };
    Ok(q)
  }

  /*
   *
   */
  fn impl_struct(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let struct_ident = &self.struct_ident;
    let q = quote::quote! {
      pub struct #struct_ident {
        pub filter: Option<Filter>,
      }
    };
    Ok(q)
  }

  fn impl_from_struct(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let ident = &self.ast.ident;
    let struct_ident  = &self.struct_ident;
    let builder_ident = &self.builder_ident;
    let q = quote::quote! {
      impl From<#ident> for #struct_ident {
        fn from(item: #ident) -> Self {
          #builder_ident::default()
          .set_filter(Filter::from(item))
          .build()
        }
      }
    };
    Ok(q)
  }

  /*
   *
   */
  fn impl_from_filter(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let struct_ident  = &self.struct_ident;
    let builder_ident = &self.builder_ident;
    let q = quote::quote! {
      impl From<Filter> for #struct_ident {
        fn from(filter: Filter) -> Self {
          #builder_ident::default()
          .set_filter(filter)
          .build()
        }
      }
    };
    Ok(q)
  }

  /*
   *
   */
  fn impl_builder(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let struct_ident  = &self.struct_ident;
    let builder_ident = &self.builder_ident;
    let q = quote::quote! {
      #[derive(Default)]
      pub struct #builder_ident {
        filter: Option<Filter>,
      }

      impl #builder_ident {
        pub fn set_filter(mut self, filter: Filter) -> #builder_ident {
          self.filter = Some(filter); self
        }

        pub fn build(self) -> #struct_ident {
          #struct_ident { filter: self.filter }
        }
      }
    };
    Ok(q)
  }
}
