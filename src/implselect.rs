

pub struct ImplSelect<'a> {
  pub ast: &'a syn::DeriveInput,
}

impl<'a> ImplSelect<'a> {
  pub fn generate(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let struct_ = self.impl_struct()?;
    let from    = self.impl_from()?;
    let builder = self.impl_builder()?;
    let q = quote::quote! {
      #struct_
      #from
      #builder
    };
    Ok(q)
  }

  /*
   *
   */
  fn impl_struct(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let q = quote::quote! {
      pub struct Select {
        pub filter: Option<Filter>,
        pub limit: Option<usize>,
        pub offset: Option<usize>,
      }
    };
    Ok(q)
  }

  /*
   *
   */
  fn impl_from(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let q = quote::quote! {
      impl From<Filter> for Select {
        fn from(filter: Filter) -> Self {
          SelectBuilder::default()
          .set_filter(filter)
          .build()
        }
      }

      impl From<SelectOne> for Select {
        fn from(select_one: SelectOne) -> Self {
          let mut b = SelectBuilder::default();
          if let Some(filter) = select_one.filter { b = b.set_filter(filter); }
          b.set_limit(1)
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
    let q = quote::quote! {
      #[derive(Default)]
      pub struct SelectBuilder {
        filter: Option<Filter>,
        limit: Option<usize>,
        offset: Option<usize>,
      }

      impl SelectBuilder {
        pub fn set_filter(mut self, filter: Filter) -> SelectBuilder {
          self.filter = Some(filter); self
        }

        pub fn set_limit(mut self, limit: usize) -> SelectBuilder {
          self.limit = Some(limit); self
        }

        pub fn set_offset(mut self, offset: usize) -> SelectBuilder {
          self.offset = Some(offset); self
        }

        pub fn build(self) -> Select {
          Select { filter: self.filter, limit: self.limit, offset: self.offset }
        }
      }
    };
    Ok(q)
  }
}
