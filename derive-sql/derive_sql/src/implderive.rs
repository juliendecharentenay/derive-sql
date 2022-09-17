use crate::SqlType;

pub struct ImplDerive<'a> {
  pub ast: &'a syn::DeriveInput,
}

impl<'a> ImplDerive<'a> {
  pub fn generate(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    self.validate()?;
    let create_table = self.impl_create_table()?;
    let select = self.impl_select()?;
    let insert = self.impl_insert()?;
    let update = self.impl_update()?;
    let delete = self.impl_delete()?;
    let r = quote::quote! {
      #create_table
      #select
      #insert
      #update
      #delete
    };
    Ok(r)
  }

  fn validate(&'a self) -> Result<(), Box<dyn std::error::Error>> {
    // Validate the type to which the derive is applied to
    match &self.ast.data {
      syn::Data::Struct(_) => {},
      syn::Data::Enum(_) => {
        return Err(Box::new(simple_error::SimpleError::new("DeriveSql macro is not supported for enum")));
      },
      syn::Data::Union(_) => {
        return Err(Box::new(simple_error::SimpleError::new("DeriveSql macro is not supported for union")));
      },
    };

    // Validate the type of fields
    // let fields = self.get_fields(); // syn::Data::Struct(syn::DataStruct { fields, .. }) = &self.ast.data;
    match self.get_fields().ok_or("Unable to retrieve fields")? {
      syn::Fields::Named(fields_named) => {
        // Check that have at least on named field
        if fields_named.named.is_empty() {
          return Err(Box::new(simple_error::SimpleError::new("DeriveSql macro does not support struct with no fields")));
        }

        // Check that all named fields have a name (ie no tuple)
        if ! fields_named.named.iter().fold(true, |r, f| r && f.ident.is_some()) {
          return Err(Box::new(simple_error::SimpleError::new("DeriveSql macro does not support fields with no name such as tuple")));
        }
      },
      syn::Fields::Unnamed(_) => {
        return Err(Box::new(simple_error::SimpleError::new("DeriveSql macro does not support Unnamed field such as Some(T)")));
      },
      syn::Fields::Unit => {
        return Err(Box::new(simple_error::SimpleError::new("DeriveSql macro does not support Unit field such as None")));
      },
    }

    Ok(())
  }

  fn get_fields(&'a self) -> Option<&syn::Fields> {
    if let syn::Data::Struct(syn::DataStruct { fields, .. }) = &self.ast.data {
      Some(fields)
    } else {
      None
    }
  }

  fn get_fields_named(&'a self) -> Option<&syn::FieldsNamed> {
    if let Some(syn::Fields::Named(fields_named)) = self.get_fields() {
      Some(fields_named)
    } else {
      None
    }
  }
  
  fn get_table_name(&'a self) -> String {
    format!("{}", self.name()).to_lowercase()
  }

  fn name(&'a self) -> &syn::Ident {
    &self.ast.ident
  }

  /*
   * output the implementation of
   *     pub fn delete(self, conn: &Connection) -> Result<Self, Box<dyn Error>>
   */
  fn impl_delete(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let name = self.name();
    let fields_named = &self.get_fields_named().ok_or("Unable to retrieve fields named")?.named;

    let statement = format!("DELETE FROM {} WHERE {} = ?", 
                            self.get_table_name(), 
                            fields_named.iter().nth(0).unwrap().ident.as_ref().unwrap());
    let parameter: &syn::Ident = fields_named.iter().nth(0).unwrap().ident.as_ref().unwrap();

    let q = quote::quote! {
      impl #name {
        pub fn delete(self, conn: &rusqlite::Connection) -> Result<Self, Box<dyn std::error::Error>> {
          conn.execute(#statement, [ &self.#parameter ])?;
          Ok(self)
        }
      }
    };
    Ok(q)
  }

  /*
   * output the implementation of
   *     pub fn update(self, conn: &Connection) -> Result<Self, Box<dyn Error>>
   */
  fn impl_update(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let name = self.name();
    let fields_named = &self.get_fields_named().ok_or("Unable to retrieve fields named")?.named;

    let statement = format!("UPDATE {} SET {} WHERE {} = ?1",
                            self.get_table_name(),
                            fields_named.iter().enumerate()
                               .map(|(i, f)| format!("{} = ?{}", f.ident.as_ref().unwrap(), i+2))
                               .fold(String::default(), |a, f| if a.is_empty() { f } else { a + ", " + f.as_str() }),
                            fields_named.iter().nth(0).unwrap().ident.as_ref().unwrap());
    let mut parameters: Vec<&syn::Ident> = fields_named.iter().map(|f| f.ident.as_ref().unwrap()).collect();
    parameters.insert(0, parameters[0]);

    let q = quote::quote! {
      impl #name {
        pub fn update(self, conn: &rusqlite::Connection) -> Result<Self, Box<dyn std::error::Error>> {
          conn.execute(#statement, ( #( &self.#parameters ),* ))?;
          Ok(self)
        }
      }
    };
    Ok(q)
  }

  /*
   * output the implement of
   *     pub fn select(conn: &Connection) -> Result<Vec<Self>, Box<dyn Error>>
   */
  fn impl_select(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let name = self.name();
    let fields_named = &self.get_fields_named().ok_or("Unable to retrieve fields named")?.named;

    let statement = format!("SELECT {} FROM {}",
                      fields_named.iter()
                        .map(|f| format!("{}", f.ident.as_ref().unwrap()))
                        .fold(String::default(), |a, f| if a.is_empty() { f } else { a + ", " + f.as_str() }),
                      self.get_table_name());
    let fields: Vec<&syn::Ident> = fields_named.iter()
                                     .map(|f| f.ident.as_ref().unwrap())
                                     .collect();
    let fields_assignment: Vec<proc_macro2::TokenStream> = fields_named.iter().enumerate()
                                     .map(|(i, _)| quote::quote! { r.get(#i)? } )
                                     .collect();

    let q = quote::quote! {
      impl #name {
        pub fn select(conn: &rusqlite::Connection) -> Result<Vec<#name>, Box<dyn std::error::Error>> {
          let mut s = conn.prepare(#statement)?;
          let i = s.query_map([], |r| Ok( #name { #( #fields : #fields_assignment ),* } ) )?;
          let r = i.collect::<Result<Vec<#name>, rusqlite::Error>>()?;
          Ok(r)
        }
      }
    };
    Ok(q)
  }

  /*
   * output the implementation of
   *    pub fn insert(self, conn: &Connection) -> Result<Self, Box<dyn Error>> 
   */
  fn impl_insert(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let name = self.name();
    let fields_named = &self.get_fields_named().ok_or("Unable to retrieve fields named")?.named;

    let statement = format!("INSERT INTO {} ({}) VALUES ({})",
                      self.get_table_name(),
                      fields_named
                          .iter()
                          .map(|f| format!("{}", f.ident.as_ref().unwrap()))
                          .fold(String::default(), |a, f| if a.is_empty() { f } else { a + ", " + f.as_str() }),
                      fields_named
                          .iter()
                          .enumerate()
                          .map(|(i, _)| format!("?{}", i+1))
                          .fold(String::default(), |a, f| if a.is_empty() { f } else { a + ", " + f.as_str() }),
                    );

    let parameters: Vec<&syn::Ident> = fields_named.iter()
                          .map(|f| f.ident.as_ref().unwrap())
                          .collect();
                         
    let q = quote::quote! {
      impl #name {
        pub fn insert(self, conn: &rusqlite::Connection) -> Result<Self, Box<dyn std::error::Error>> {
          conn.execute(#statement, ( #( &self.#parameters ),* ))?;
          Ok(self)
        }
      }
    };
    Ok(q)
  }

  /*
   * output the implementation the "create_table" function
   */
  fn impl_create_table(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let name = self.name();

    let fields_statement = self.get_fields_named()
      .ok_or("Unable to retrieve FieldsNamed")?
      .named
      .iter()
      .fold(String::default(),
            |statement, field| {
              let sql_type = SqlType::from_type(&field.ty);
              if matches!(sql_type, SqlType::Unsupported) {
                statement
              } else if statement.is_empty() {
                // format!("{} {} PRIMARY KEY", field.ident.as_ref().unwrap(), SqlType::to_string(&sql_type))
                format!("{} {}", field.ident.as_ref().unwrap(), SqlType::to_string(&sql_type))
              } else {
                format!("{}, {} {}", statement, field.ident.as_ref().unwrap(), SqlType::to_string(&sql_type))
              }
            });
      
    let create_table_statement = format!("CREATE TABLE {} ( {} )", self.get_table_name(), fields_statement);
             
    let r = quote::quote! {
      impl #name {
        pub fn create_table(conn: &rusqlite::Connection) -> Result<(), Box<dyn std::error::Error>> {
          conn.execute(#create_table_statement, ())?;
          Ok(())
        }
      }
    };
    Ok(r)
  }
}

