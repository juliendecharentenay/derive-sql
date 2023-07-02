pub struct Derive {
  ast: syn::DeriveInput,
}

impl Derive {
  pub fn new(ast: syn::DeriveInput) -> Derive { Derive { ast } }

  pub fn generate(&self) -> syn::parse::Result<proc_macro2::TokenStream> {
    let fields_named 
    = if let syn::Data::Struct(syn::DataStruct { fields, .. }) = &self.ast.data {
      if let syn::Fields::Named(fields_named) = fields {
        fields_named
      } else {
        return Err(syn::parse::Error::new(self.ast.ident.span(), "Derive macro support named fields only"));
      }
    } else {
      return Err(syn::parse::Error::new(self.ast.ident.span(), "Derive macro is only available on Struct"));
    };

    let name = &self.ast.ident;
    let meta: FromAttributesMeta = (&self.ast.attrs).try_into()?;
    if meta.ident.is_none() {
      return Err(syn::parse::Error::new(self.ast.ident.span(), r#"Missing helper attribute from_attributes. Add `#[from_attributes(ident = "...")]` with ... replaced withe name of the helper attribute"#));
    }

    let ident = meta.ident.unwrap();
    let variables = fields_named.named.iter()
      .map(|f| f.ident.clone() );

    let (idents, initialisation, statements) = fields_named.named.iter()
      .map(|f| {
        let ident = f.ident.as_ref().ok_or(syn::parse::Error::new(self.ast.ident.span(), "Unable to find ident"))?;
        let ty: Type = (&f.ty).into();
        let (init, statement) = match ty {
          Type::Boolean => (
               quote::quote! { let mut #ident = false; },
               quote::quote! { #ident = true; },
               ),
          Type::Option => (
               quote::quote! { let mut #ident = None; },
               quote::quote! { #ident = Some(meta.value()?.parse()?); },
               ),
          Type::Unsupported => {return Err(syn::parse::Error::new(ident.span(), format!("Ident {} type is not supported", ident)));},
        };
        Ok((ident.clone(), init, statement))
      })
      .collect::<syn::parse::Result<Vec<(syn::Ident, proc_macro2::TokenStream, proc_macro2::TokenStream)>>>()?
      .into_iter()
      .fold((Vec::new(), Vec::new(), Vec::new()),
            |(mut idents, mut inits, mut stmts), (ident, init, stmt)| {
              idents.push(ident); inits.push(init); stmts.push(stmt);
              (idents, inits, stmts)
            });
    let assignement = idents.iter().zip(statements.iter())
        .fold(None,
              |acc, (ident, statement)| {
                let ident_str = ident.to_string();
                let r = match acc {
                  None => {
                    quote::quote! {
                      if meta.path.is_ident(#ident_str) {
                        #statement
                        Ok(())
                      }
                    }
                  },
                  Some(s) => {
                    quote::quote! {
                      #s
                      else if meta.path.is_ident(#ident_str) {
                        #statement
                        Ok(())
                      }
                    }
                  },
                };
                Some(r)
              });
    if assignement.is_none() {
      return Err(syn::parse::Error::new(self.ast.ident.span(), "Struct needs to have at least one member"));
    }
    let assignement = assignement.unwrap();
    let assignement = quote::quote! {
      #assignement
      else {
        Err(meta.error("Unsupported property"))
      }
    };
    
    let r = quote::quote! {
      impl TryFrom<&syn::Field> for #name {
        type Error = syn::parse::Error;
        fn try_from(i: &syn::Field) -> Result<#name, syn::parse::Error> {
          (&i.attrs).try_into()
        }
      }

      impl TryFrom<&syn::DeriveInput> for #name {
        type Error = syn::parse::Error;
        fn try_from(i: &syn::DeriveInput) -> Result<#name, syn::parse::Error> {
          (&i.attrs).try_into()
        }
      }

      impl TryFrom<&Vec<syn::Attribute>> for #name {
        type Error = syn::parse::Error;
        fn try_from(attrs: &Vec<syn::Attribute>) -> Result<#name, syn::parse::Error> {
          #( #initialisation )*
          for attr in attrs.iter().filter(|attr| attr.path().is_ident(#ident)) {
            attr.parse_nested_meta(|meta| {
              #assignement
            })?;
          }
          Ok( #name { #( #variables ),* } )
        }
      }
    };
    Ok(r)
  }
}

enum Type {
  Boolean,
  Option,
  Unsupported,
}

impl From<&syn::Type> for Type {
  fn from(ty: &syn::Type) -> Type {
    match ty {
      syn::Type::Path(syn::TypePath { path, .. }) if path.is_ident("bool") => Type::Boolean,
      syn::Type::Path(syn::TypePath { path: syn::Path { segments, .. }, .. })
      if segments.first().and_then(|p| Some(p.ident == "Option")).unwrap_or(false) => Type::Option,
      _ => Type::Unsupported,
    }
  }
}

// Extract struct from `from_attributes`
struct FromAttributesMeta {
  pub ident: Option<syn::LitStr>,
}

impl TryFrom<&Vec<syn::Attribute>> for FromAttributesMeta {
  type Error = syn::parse::Error;

  fn try_from(attrs: &Vec<syn::Attribute>) -> Result<FromAttributesMeta, syn::parse::Error> {
    let mut ident = None;
    for attr in attrs.iter().filter(|attr| attr.path().is_ident("from_attributes")) {
      attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("ident") {
          ident = Some(meta.value()?.parse()?);
          Ok(())
        } else {
          Err(meta.error("Unsupported property"))
        }
      })?;
    }
    Ok(FromAttributesMeta { ident })
  }
}

