use crate::utility;
use crate::SqlType;

pub struct ImplDerive<'a> {
  pub ast: &'a syn::DeriveInput,
}

impl<'a> ImplDerive<'a> {
  pub fn generate(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    self.validate()?;
    let o_filter     = crate::implfilter::ImplFilter { ast: self.ast }.generate()?;
    let o_select     = crate::implselect::ImplSelect { ast: self.ast }.generate()?;
    let o_select_one = crate::implfilterwrapper::ImplFilterWrapper { 
                                ast: self.ast, 
                                struct_ident: syn::Ident::new("SelectOne", self.name().span()),
                                builder_ident: syn::Ident::new("SelectOneBuilder", self.name().span()),
                       }.generate()?;
    let o_count      = crate::implfilterwrapper::ImplFilterWrapper { 
                                ast: self.ast, 
                                struct_ident: syn::Ident::new("Count", self.name().span()),
                                builder_ident: syn::Ident::new("CountBuilder", self.name().span()),
                       }.generate()?;
    let o_delete     = crate::implfilterwrapper::ImplFilterWrapper { 
                                ast: self.ast, 
                                struct_ident: syn::Ident::new("Delete", self.name().span()),
                                builder_ident: syn::Ident::new("DeleteBuilder", self.name().span()),
                       }.generate()?;
    let enum_        = self.impl_enum()?;
    let struct_sql   = self.impl_struct_sql()?;
    let constructor  = self.impl_constructor()?;
    let create_table = self.impl_create_table()?;
    let delete_table = self.impl_delete_table()?;
    let table_exists = self.impl_table_exists()?;
    let select_all   = self.impl_select_all()?;
    let select_one   = self.impl_select_one()?;
    let select       = self.impl_select()?;
    let count_all    = self.impl_count_all()?;
    let count        = self.impl_count()?;
    let insert       = self.impl_insert()?;
    let update_to    = self.impl_update_to()?;
    let update       = self.impl_update()?;
    let delete_all   = self.impl_delete_all()?;
    let delete       = self.impl_delete()?;
    let r = quote::quote! {
      #o_filter
      #o_select
      #o_select_one
      #o_count
      #o_delete
      #enum_
      #struct_sql
      #constructor
      #create_table
      #delete_table
      #table_exists
      #select_all
      #select_one
      #select
      #count_all
      #count
      #insert
      #update_to
      #update
      #delete_all
      #delete
    };
    Ok(r)
  }

  fn validate(&'a self) -> Result<(), Box<dyn std::error::Error>> {
    // Validate the type to which the derive is applied to
    match &self.ast.data {
      syn::Data::Struct(_) => {},
      syn::Data::Enum(_) => {
        return Err("DeriveSql macro is not supported for enum".into());
      },
      syn::Data::Union(_) => {
        return Err("DeriveSql macro is not supported for union".into());
      },
    };

    // Validate the type of fields
    match utility::get_fields(self.ast).ok_or("Unable to retrieve fields")? {
      syn::Fields::Named(fields_named) => {
        // Check that have at least on named field
        if fields_named.named.is_empty() {
          return Err("DeriveSql macro does not support struct with no fields".into());
        }

        // Check that all named fields have a name (ie no tuple)
        if ! fields_named.named.iter().all(|f| f.ident.is_some()) {
          return Err("DeriveSql macro does not support fields with no name such as tuple".into());
        }
      },
      syn::Fields::Unnamed(_) => {
        return Err("DeriveSql macro does not support Unnamed field such as Some(T)".into());
      },
      syn::Fields::Unit => {
        return Err("DeriveSql macro does not support Unit field such as None".into());
      },
    }

    // Validate that there no more than one named field marked with #[derive_sql(primary_key)]
    match utility::get_fields(self.ast).ok_or("Unabl to retrieve fields")? {
      syn::Fields::Named(fields_named) => {
        let count = fields_named.named.iter().filter(|f| Self::field_has_primary_key(f)).count();
        if count > 1 {
          return Err(format!("{} fields have been marked with #[derive_sql(primary_key)]. Only 1 field is allowed to be marked as such", count).into());
        }
      },
      _ => {
        return Err("DeriveSql macro only supports Named field".into());
      },
    }

    Ok(())
  }

  fn get_table_name(&'a self) -> String {
    format!("{}", self.name())
  }

  fn name(&'a self) -> &syn::Ident {
    &self.ast.ident
  }

  fn name_sql(&'a self) -> syn::Ident {
    syn::Ident::new(format!("{}Sql", self.name()).as_str(), self.name().span())
  }

  fn field_has_primary_key(f: &syn::Field) -> bool {
    Self::field_has(f, "primary_key")
  }

  fn field_has(f: &syn::Field, ident: &str) -> bool {
    f.attrs.iter()
    .filter(|a| a.path().get_ident().map(|i| i.eq("derive_sql")).unwrap_or(false) )
    .filter(|a| {
      let mut r = false;
      a.parse_nested_meta(|meta| { r = r || meta.path.is_ident(ident); Ok(())});
      r
    })
    .count() > 0
  }

  fn field_on(f: &syn::Field, ident: &str) -> Option<String> {
    Self::get_attr(&f.attrs, ident)
  }

  fn get_attr(attrs: &Vec<syn::Attribute>, ident: &str) -> Option<String> {
    attrs.iter()
    .filter(|a| a.path().get_ident().map(|i| i.eq("derive_sql")).unwrap_or(false) )
    .map(|a| {
      let mut r = None;
      a.parse_nested_meta(|meta| { 
        if r.is_none() {
          if meta.path.is_ident(ident) {
            if let Ok(value) = meta.value() {
              if let Ok(s) = value.parse::<syn::LitStr>() {
                r = Some(s.value().to_string());
              }
            }
          }
        }
        Ok(())
      });
      r
    })
    .filter(|a| a.is_some())
    .fold(None, |acc, a| acc.or(a))
  }

  /*
   * output an enum to store the SQL connection type
   *  enum SqlConnection {
   *       Rusqlite(&'a rusqlite::Connection)
   *  }
   */
  fn impl_enum(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let doc = "This enum provides an identification of the different SQL wrapper supported";
    let q = quote::quote! {
      #[doc = #doc]
      enum SqlConnection<'a> {
        Rusqlite(&'a rusqlite::Connection),
        RusqliteOwned(rusqlite::Connection),
      }
    };
    Ok(q)
  }

  /*
   * output the AbcSql Struct:
   *  pub struct AbcSql<'a> {
   *    connection: SqlConnection<'a>,
   *  }
   */
  fn impl_struct_sql(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let name_sql = self.name_sql();
    let doc = format!(r#"
The struct {} wraps around the connection to the SQL database - specified as part of the constructor -
and manages the interaction between the software and the database
"#, name_sql); 
    let q = quote::quote! {
      #[doc = #doc]
      pub struct #name_sql<'a> {
        connection: SqlConnection<'a>,
      }
    };

    Ok(q)
  }

  /*
   * output the AbcSql constructor:
   *   fn from_rusqlite(conn) -> Result<AbcSql, Box<dyn Error>>
   */
  fn impl_constructor(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let name_sql = self.name_sql();
    let doc = format!(r#"
Construct a SQL connector to manipulate struct of type {} to an SQLite database using the rusqlite wrapper
"#, self.name());
    let mut q = quote::quote! {
      #[doc = #doc]
      impl<'a> #name_sql<'a> {
        pub fn from_rusqlite(conn: &'a rusqlite::Connection) -> Result<#name_sql<'a>, Box<dyn std::error::Error>> {
          Ok( #name_sql { connection: SqlConnection::Rusqlite(conn) } )
        }
      }
    };

    if let Some(f) = Self::get_attr(&self.ast.attrs, "rusqlite_connection") {
      let f_ident = syn::Ident::new(f.as_str(), proc_macro2::Span::call_site());
      let doc = format!(r#"
Construct a default SQL connector to manipulate struct of type {name_sql} to an SQLite database stored in file {f}
"#);
      q = quote::quote! {
        #q

        #[doc = #doc]
        impl<'a> #name_sql<'a> {
          pub fn from_database() -> Result<#name_sql<'a>, Box<dyn std::error::Error>> {
            Ok( #name_sql { connection: SqlConnection::RusqliteOwned(#f_ident()?) } )
          }
        }
      };
    }

    Ok(q)
  }

  /*
   * output the implementation of
   *     pub fn delete_all(&self) -> Result<(), Box<dyn Error>>
   */
  fn impl_delete_all(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let name_sql = self.name_sql();
    let q = quote::quote! {
      impl<'a> #name_sql<'a> {
        pub fn delete_all(&self) -> Result<(), Box<dyn std::error::Error>> {
          self.delete(DeleteBuilder::default().build())
        }
      }
    };
    Ok(q)
  }

  /*
   * output the implementation of
   *     pub fn delete(&self, delete: Delete) -> Result<(), Box<dyn Error>>
   */
  fn impl_delete(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let name_sql = self.name_sql();
    let statement = format!("DELETE FROM {}", self.get_table_name());

    let q = quote::quote! {
      impl<'a> #name_sql<'a> {
        pub fn delete(&self, delete: Delete) -> Result<(), Box<dyn std::error::Error>> {
          let sqlite_statement = || {
            let mut statement = #statement.to_string();
            if let Some(filter) = &delete.filter {
              statement += format!(" WHERE {}", Filter::to_condition(&filter)).as_str();
            }
            statement
          };
          match &self.connection {
            SqlConnection::Rusqlite(conn)      => { conn.execute(sqlite_statement().as_str(), ())?; },
            SqlConnection::RusqliteOwned(conn) => { conn.execute(sqlite_statement().as_str(), ())?; },
          };
          Ok(())
        }
      }
    };
    Ok(q)
  }

  /*
   * output the implementation of
   *     pub fn update(&self, primary_key: &Type, value: Abc) -> Result<Abc, Box<dyn Error>>
   *
   * Only done if one field is marked using `#[derive_sql(primary_key)]`
   */
  fn impl_update(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let name     = self.name();
    let name_sql = self.name_sql();
    let fields_named = &utility::get_fields_named(self.ast).ok_or("Unable to retrieve fields named")?.named;
    if let Some(primary_key) = fields_named.iter().find(|f| Self::field_has_primary_key(f)) {
      let primary_key_ident = &primary_key.ident;
      let primary_key_ty    = &primary_key.ty;
      let statement = format!("UPDATE {} SET {} WHERE {}",
                            self.get_table_name(),
                            fields_named.iter().enumerate()
                               .map(|(i, f)| format!("{} = ?{}", f.ident.as_ref().unwrap(), i+2))
                               .fold(String::default(), |a, f| if a.is_empty() { f } else { a + ", " + f.as_str() }),
                               format!("{} = ?{}", primary_key.ident.as_ref().unwrap(), 1));
      let parameters: Vec<&syn::Ident> = fields_named.iter().map(|f| f.ident.as_ref().unwrap()).collect();
      let updates: Vec<proc_macro2::TokenStream> = fields_named.iter()
          .filter(|f| Self::field_on(f, "on_update").is_some() || Self::field_on(f, "on_insert_update").is_some())
          .map(|f| {
            let ident = &f.ident;
            let f_ident = syn::Ident::new(Self::field_on(f, "on_update").or_else(|| Self::field_on(f, "on_insert_update")).unwrap().as_str(), proc_macro2::Span::call_site());
            quote::quote! {
              value.#ident = #f_ident()?;
            }
          }).collect();
                         

      let q = quote::quote! {
        impl<'a> #name_sql<'a> {
          pub fn update(&self, primary_key: #primary_key_ty, mut value: #name) -> Result<#name, Box<dyn std::error::Error>> {
            #( #updates )*
            value.#primary_key_ident = primary_key;
            match &self.connection {
              SqlConnection::Rusqlite(conn)      => { conn.execute(#statement, ( &value.#primary_key_ident, #( &value.#parameters ),* ))?; },
              SqlConnection::RusqliteOwned(conn) => { conn.execute(#statement, ( &value.#primary_key_ident, #( &value.#parameters ),* ))?; },
            };
            Ok(value)
          }
        }
      };
      Ok(q)
    } else {
      Ok(quote::quote! { })
    }
  }

  /*
   * output the implementation of
   *     pub fn update_to(&self, from: &Abc, to: &Abc) -> Result<(), Box<dyn Error>>
   */
  fn impl_update_to(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let name     = self.name();
    let name_sql = self.name_sql();
    let fields_named = &utility::get_fields_named(self.ast).ok_or("Unable to retrieve fields named")?.named;

    let statement = format!("UPDATE {} SET {} WHERE {}",
                            self.get_table_name(),
                            fields_named.iter().enumerate()
                               .map(|(i, f)| format!("{} = ?{}", f.ident.as_ref().unwrap(), i+1+fields_named.len()))
                               .fold(String::default(), |a, f| if a.is_empty() { f } else { a + ", " + f.as_str() }),
                            fields_named.iter().enumerate()
                               .map(|(i, f)| format!("{} = ?{}", f.ident.as_ref().unwrap(), i+1))
                               .fold(String::default(), |a, f| if a.is_empty() { f } else { a + " AND " + f.as_str() }));
    let parameters: Vec<&syn::Ident> = fields_named.iter().map(|f| f.ident.as_ref().unwrap()).collect();

    let q = quote::quote! {
      impl<'a> #name_sql<'a> {
        pub fn update_to(&self, from: &#name, to: &#name) -> Result<(), Box<dyn std::error::Error>> {
          match &self.connection {
            SqlConnection::Rusqlite(conn)      => { conn.execute(#statement, ( #( &from.#parameters ),* , #( &to.#parameters ),* ))?; },
            SqlConnection::RusqliteOwned(conn) => { conn.execute(#statement, ( #( &from.#parameters ),* , #( &to.#parameters ),* ))?; },
          };
          Ok(())
        }
      }
    };
    Ok(q)
  }

  /*
   * output the implementation of
   *     pub fn select_all(&self) -> Result<Vec<Abc>, Box<dyn Error>>
   */
  fn impl_select_all(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let name     = self.name();
    let name_sql = self.name_sql();
    let q = quote::quote! {
      impl<'a> #name_sql<'a> {
        pub fn select_all(&self) -> Result<Vec<#name>, Box<dyn std::error::Error>> {
          self.select(SelectBuilder::default().build())
        }
      }
    };
    Ok(q)
  }

  /*
   * output the implementation of
   *     pub fn select_one(&self, select_one: SelectOne) -> Result<Option<Abc>, Box<dyn Error>>
   */
  fn impl_select_one(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let name     = self.name();
    let name_sql = self.name_sql();
    let q = quote::quote! {
      impl<'a> #name_sql<'a> {
        pub fn select_one(&self, select_one: SelectOne) -> Result<Option<#name>, Box<dyn std::error::Error>> {
          let r = self.select(select_one.into())?;
          Ok(r.into_iter().nth(0))
        }
      }
    };
    Ok(q)
  }


  /*
   * output the implementation of
   *     pub fn select(&self, select: Select) -> Result<Vec<Abc>, Box<dyn Error>>
   */
  fn impl_select(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let name     = self.name();
    let name_sql = self.name_sql();
    let fields_named = &utility::get_fields_named(self.ast).ok_or("Unable to retrieve fields named")?.named;

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
      impl<'a> #name_sql<'a> {
        pub fn select(&self, select: Select) -> Result<Vec<#name>, Box<dyn std::error::Error>> {
          let sqlite_statement = || {
              let mut statement = #statement.to_string();
              if let Some(filter) = &select.filter {
                statement += format!(" WHERE {}", Filter::to_condition(&filter)).as_str();
              }
              if let Some(limit) = &select.limit {
                statement += format!(" LIMIT {}", limit).as_str();
              }
              if let Some(offset) = &select.offset {
                statement += format!(" ORDER BY ( SELECT NULL ) OFFSET {}", offset).as_str();
              }
              statement
          };
          let r = match &self.connection {
            SqlConnection::Rusqlite(conn) => {
              conn.prepare(sqlite_statement().as_str())?
              .query_map([], |r| Ok( #name { #( #fields : #fields_assignment ),* } ) )?
              .collect::<Result<Vec<#name>, rusqlite::Error>>()?
            },
            SqlConnection::RusqliteOwned(conn) => {
              conn.prepare(sqlite_statement().as_str())?
              .query_map([], |r| Ok( #name { #( #fields : #fields_assignment ),* } ) )?
              .collect::<Result<Vec<#name>, rusqlite::Error>>()?
            },
          };
          Ok(r)
        }
      }
    };
    Ok(q)
  }

  /*
   * output the implementation of
   *    pub fn count_all(&self) -> Result<usize, Box<dyn Error>>
   */
  fn impl_count_all(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let name_sql = self.name_sql();
    let q = quote::quote! {
      impl<'a> #name_sql<'a> {
        pub fn count_all(&self) -> Result<usize, Box<dyn std::error::Error>> {
          self.count(CountBuilder::default().build())
        }
      }
    };
    Ok(q)
  }

  /*
   * output the implementation of
   *    pub fn count(&self, count: Count) -> Result<usize, Box<dyn Error>>
   */
  fn impl_count(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let name_sql = self.name_sql();
    let fields_named = &utility::get_fields_named(self.ast).ok_or("Unable to retrieve fields named")?.named;

    let statement = format!("SELECT COUNT( {} ) FROM {}",
                      fields_named.iter().next().unwrap().ident.as_ref().unwrap(),
                      self.get_table_name());

    let q = quote::quote! {
      impl<'a> #name_sql<'a> {
        pub fn count(&self, count: Count) -> Result<usize, Box<dyn std::error::Error>> {
          let sqlite_statement = || {
              let mut statement = #statement.to_string();
              if let Some(filter) = count.filter {
                statement += format!(" WHERE {}", Filter::to_condition(&filter)).as_str();
              }
              statement
          };
          let r = match &self.connection {
            SqlConnection::Rusqlite(conn) => {
              conn.prepare(sqlite_statement().as_str())?
              .query_map([], |r| r.get(0))?
              .collect::<Result<Vec<usize>, rusqlite::Error>>()?
            },
            SqlConnection::RusqliteOwned(conn) => {
              conn.prepare(sqlite_statement().as_str())?
              .query_map([], |r| r.get(0))?
              .collect::<Result<Vec<usize>, rusqlite::Error>>()?
            },
          };
          if r.len() == 1 { 
            Ok(r[0])
          } else {
            Err(format!("SELECT COUNT result is expected to have length of 1. Current result is {:?}", r).into()) 
          }
        }
      }
    };
    Ok(q)
  }
  
  /*
   * output the implementation of
   *    pub fn insert(&self, o: &Abc) -> Result<(), Box<dyn Error>> 
   */
  fn impl_insert(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let name     = self.name();
    let name_sql = self.name_sql();
    let fields_named = &utility::get_fields_named(self.ast).ok_or("Unable to retrieve fields named")?.named;

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
    let updates: Vec<proc_macro2::TokenStream> = fields_named.iter()
          .filter(|f| Self::field_on(f, "on_insert").is_some() || Self::field_on(f, "on_insert_update").is_some())
          .map(|f| {
            let ident = &f.ident;
            let f_ident = syn::Ident::new(Self::field_on(f, "on_insert").or_else(|| Self::field_on(f, "on_insert_update")).unwrap().as_str(), proc_macro2::Span::call_site());
            quote::quote! {
              i.#ident = #f_ident()?;
            }
          }).collect();
                         
    let q = quote::quote! {
      impl<'a> #name_sql<'a> {
        pub fn insert(&self, mut i: #name) -> Result<#name, Box<dyn std::error::Error>> {
          #( #updates )*
          match &self.connection {
            SqlConnection::Rusqlite(conn)      => { conn.execute(#statement, ( #( &i.#parameters ),* ))?; },
            SqlConnection::RusqliteOwned(conn) => { conn.execute(#statement, ( #( &i.#parameters ),* ))?; },
          };
          Ok(i)
        }
      }
    };
    Ok(q)
  }

  /*
   * Output the implementation of the "table_exists" function
   *   pub fn table_exists(&self) -> Result<bool, Box<dyn Error>>
   */
  fn impl_table_exists(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let name_sql = self.name_sql();
    // let statement = format!("SELECT * FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_NAME = '{}'", self.get_table_name());
    let statement = format!("SELECT * FROM sqlite_master WHERE name='{}'", self.get_table_name());
    let r = quote::quote! {
      impl<'a> #name_sql<'a> {
        pub fn table_exists(&self) -> Result<bool, Box<dyn std::error::Error>> {
          let r = match &self.connection {
            SqlConnection::Rusqlite(conn)      => { conn.prepare(#statement)?.exists([])?  },
            SqlConnection::RusqliteOwned(conn) => { conn.prepare(#statement)?.exists([])?  },
          };
          Ok(r)
        }
      }
    };
    Ok(r)
  }

  /*
   * Output the implementation of
   *   pub fn delete_table(&self) -> Result<(), Box<dyn Error>>
   */
  fn impl_delete_table(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let name_sql = self.name_sql();
    let statement = format!("DROP TABLE {}", self.get_table_name());
    let r = quote::quote! {
      impl<'a> #name_sql<'a> {
        pub fn delete_table(&self) -> Result<(), Box<dyn std::error::Error>> {
          match &self.connection {
            SqlConnection::Rusqlite(conn)      => { conn.execute(#statement, ())?; },
            SqlConnection::RusqliteOwned(conn) => { conn.execute(#statement, ())?; },
          }
          Ok(())
        }
      }
    };
    Ok(r)
  }

  /*
   * output the implementation the "create_table" function
   *   pub fn create_table(&self ) -> Result<(), Box<dyn Error>>
   */
  fn impl_create_table(&'a self) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let name_sql = self.name_sql();

    let fields_statement = utility::get_fields_named(self.ast)
      .ok_or("Unable to retrieve FieldsNamed")?
      .named
      .iter()
      .fold(String::default(),
            |statement, field| {
              let sql_type = SqlType::from_type(&field.ty);
              let st = if matches!(sql_type, SqlType::Unsupported) {
                "".to_string()
              } else if Self::field_has_primary_key(&field) {
                format!("{} {} PRIMARY KEY", field.ident.as_ref().unwrap(), SqlType::to_string(&sql_type))
              } else {
                format!("{} {}", field.ident.as_ref().unwrap(), SqlType::to_string(&sql_type))
              };

              if statement.is_empty() { st }
              else if st.is_empty() { statement }
              else { format!("{statement}, {st}") }
            });
      
    let statement = format!("CREATE TABLE IF NOT EXISTS {} ( {} )", self.get_table_name(), fields_statement);
             
    let q = quote::quote! {
      impl<'a> #name_sql<'a> {
        pub fn create_table(&self) -> Result<(), Box<dyn std::error::Error>> {
          match &self.connection {
            SqlConnection::Rusqlite(conn)      => { conn.execute(#statement, ())?; },
            SqlConnection::RusqliteOwned(conn) => { conn.execute(#statement, ())?; },
          }
          Ok(())
        }
      }
    };
    Ok(q)
  }
}

