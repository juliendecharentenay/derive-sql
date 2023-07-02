use syn::parse;

use crate::{attributes, input, input::Input};

mod filter;
mod selector;
mod sqltype; use sqltype::SqlType;

pub struct DeriveSql<'a> {
  ast: &'a syn::DeriveInput,
  attrs: attributes::sql::Attributes,
}

impl<'a> DeriveSql<'a> {
  pub fn new(ast: &'a syn::DeriveInput) -> parse::Result<DeriveSql> {
    let attrs = ast.try_into()?;
    Ok( DeriveSql { ast, attrs } )
  }

  pub fn generate(self) -> parse::Result<proc_macro2::TokenStream> {
    let input = self.input();

    // Checks
    //  1- All fields are supported SqlTypes
    input.fields()?.iter()
      .map(|field| 
        if matches!(SqlType::from(field), SqlType::Unsupported) {
          Err(syn::parse::Error::new(field.span(), "Field type is not supported"))
        } else {
          Ok(())
        }
      )
      .collect::<parse::Result<Vec<()>>>()?;

    // Collect pieces of codes
    let enum_connection  = sql::impl_connection()?;
    let sql_constructor  = self.impl_sql_constructor()?;
    let sql_create_table = self.impl_sql_create_table()?;
    let sql_delete_table = self.impl_sql_delete_table()?;
    let sql_table_exists = self.impl_sql_table_exists()?;
    let sql_insert       = self.impl_sql_insert()?;
    let sql_update       = self.impl_sql_update()?;
    let sql_delete       = self.impl_sql_delete()?;
    let sql_select       = self.impl_sql_select()?;
    let selector         = selector::Selector::new(self.ast).generate()?;

    // Combine and output final code
    let class_sql = input.class_sql();
    let visibility = input.visibility();
    let doc = format!(r#"
The struct {} wraps around the connection to the SQL database - specified as part of the constructor -
and manages the interaction between the software and the database
"#, class_sql); 
    Ok(quote::quote! {
      pub mod sql {
        use super::*;
        #enum_connection
        #selector
      }

      #[doc = #doc]
      #visibility struct #class_sql<'a> {
        connection: sql::Connection<'a>,
      }
      impl<'a> #class_sql<'a> {
        #sql_constructor
        #sql_create_table
        #sql_delete_table
        #sql_table_exists
        #sql_insert
        #sql_select
        #sql_update
        #sql_delete
      }
    })
  }

  fn input(&self) -> Input                 { Input::from(self.ast) }
  // fn class_sql(&self) -> syn::Ident        { let input = self.input(); syn::Ident::new(format!("{}Sql", input.class()).as_str(), input.class().span()) }
  fn table_name(&self) -> String           { let input = self.input(); format!("{}", input.class()) }

  /*
   * output the implementation of
   *     pub fn delete(&self, delete: Delete) -> Result<(), Box<dyn Error>>
   */
  fn impl_sql_delete(&self) -> parse::Result<proc_macro2::TokenStream> {
    let statement = format!("DELETE FROM {} ", self.table_name());
    Ok(quote::quote! {
      pub fn delete_all(&self) -> Result<(), Box<dyn std::error::Error>> { self.delete(().try_into()?) }
      pub fn delete(&self, select: sql::Selector) -> Result<(), Box<dyn std::error::Error>> {
        let sqlite_statement = #statement.to_string() + select.to_statement()?.as_str();
        match &self.connection {
          sql::Connection::Rusqlite(conn)      => { conn.execute(sqlite_statement.as_str(), ())?; },
          sql::Connection::RusqliteOwned(conn) => { conn.execute(sqlite_statement.as_str(), ())?; },
        };
        Ok(())
      }
    })
  }

  /*
   * output the implementation of
   *     pub fn update(&self, s: Selector, i: Abc) -> Result<Abc, Box<dyn Error>>
   */
  fn impl_sql_update(&self) -> syn::parse::Result<proc_macro2::TokenStream> {
    let input = self.input();
    let class = input.class();

    // Update to fields when inserted
    let updates: Vec<proc_macro2::TokenStream> = Input::from(self.ast).fields()?.iter()
        .map(|f| { let ident = f.ident()?; Ok( f.on_update().map(|f| quote::quote! { i.#ident = #f()?; }) ) })
        .filter(|r| r.as_ref().map(|s| s.is_some()).unwrap_or(true)).map(|r| r.map(|s| s.unwrap()) )
        .collect::<parse::Result<Vec<proc_macro2::TokenStream>>>()?;

    let statement = format!("UPDATE {} SET {} ", self.table_name(),
                            input.idents()?.iter().enumerate()
                            .map(|(i, ident)| format!("{} = ?{}", ident, i+1) )
                            .collect::<Vec<String>>().join(", "));
    let idents = input.idents()?;

    Ok(quote::quote! {
      pub fn update(&self, select: sql::Selector, mut i: #class) -> Result<#class, Box<dyn std::error::Error>> {
        #( #updates )*
        let sqlite_statement = #statement.to_string() + select.to_statement()?.as_str();
        match &self.connection {
          sql::Connection::Rusqlite(conn)      => { conn.execute(sqlite_statement.as_str(), ( #( &i.#idents ),* ))?; },
          sql::Connection::RusqliteOwned(conn) => { conn.execute(sqlite_statement.as_str(), ( #( &i.#idents ),* ))?; },
        };
        Ok(i)
      }
    })
  }

  /*
   * output the implementation of
   *     pub fn select(&self, select: Select) -> Result<Vec<Abc>, Box<dyn Error>>
   */
  fn impl_sql_select(&self) -> syn::parse::Result<proc_macro2::TokenStream> {
    let input = self.input();
    let class = input.class(); let fields = input.fields()?;

    // SQL Select statment
    let statement = format!("SELECT {} FROM {} ",
         fields.iter().map(|f| Ok(f.ident_str()?)).collect::<parse::Result<Vec<String>>>()?.join(", "),
         self.table_name());

    // Fields allocation
    let idents = input.idents()?;
    let assignments = fields.iter().enumerate().map(|(i, _)| quote::quote! { r.get(#i)? } ).collect::<Vec<proc_macro2::TokenStream>>();
    Ok(quote::quote! {
        pub fn select_one(&self, select: sql::Selector) -> Result<Option<#class>, Box<dyn std::error::Error>> {
          Ok(self.select(select.with_limit(1))?.into_iter().nth(0))
        }
        pub fn select_all(&self) -> Result<Vec<#class>, Box<dyn std::error::Error>> { self.select(().try_into()?) }
        pub fn select(&self, select: sql::Selector) -> Result<Vec<#class>, Box<dyn std::error::Error>> {
          let sqlite_statement = #statement.to_string() + select.to_statement()?.as_str();
          let r = match &self.connection {
            sql::Connection::Rusqlite(conn)      => conn.prepare(sqlite_statement.as_str())?,
            sql::Connection::RusqliteOwned(conn) => conn.prepare(sqlite_statement.as_str())?,
          }
          .query_map([], |r| Ok( #class { #( #idents : #assignments ),* } ) )?
          .collect::<Result<Vec<#class>, rusqlite::Error>>()?;
          Ok(r)
        }
    })
  }

  /*
   * output the implementation of
   *    pub fn insert(&self, o: &Abc) -> Result<(), Box<dyn Error>> 
   */
  fn impl_sql_insert(&self) -> parse::Result<proc_macro2::TokenStream> {
    let input = self.input();
    let class = input.class();

    // Update to fields when inserted
    let updates: Vec<proc_macro2::TokenStream> = Input::from(self.ast).fields()?.iter()
        .map(|f| { let ident = f.ident()?; Ok( f.on_insert().map(|f| quote::quote! { i.#ident = #f()?; }) ) })
        .filter(|r| r.as_ref().map(|s| s.is_some()).unwrap_or(true)).map(|r| r.map(|s| s.unwrap()) )
        .collect::<parse::Result<Vec<proc_macro2::TokenStream>>>()?;

    // SQL statement
    let rusqlite_statement = format!("INSERT INTO {} ({}) VALUES ({})", self.table_name(),
         Input::from(self.ast).fields()?.iter().map(|f| Ok(f.ident_str()?)).collect::<parse::Result<Vec<String>>>()?.join(", "),
         Input::from(self.ast).fields()?.iter().enumerate().map(|(i, _)| format!("?{}", i+1)).collect::<Vec<String>>().join(", "));
    let parameters: Vec<proc_macro2::TokenStream> = Input::from(self.ast).fields()?.iter().map(|f| {let p = f.ident()?; Ok(quote::quote! { &i.#p })})
        .collect::<parse::Result<Vec<proc_macro2::TokenStream>>>()?;
    let execute = sql::execute(rusqlite_statement, parameters)?;

    Ok(quote::quote! {
      pub fn insert(&self, mut i: #class) -> Result<#class, Box<dyn std::error::Error>> { #( #updates )* #execute; Ok(i) }
    })
  }

  /*
   * Output the implementation of the "table_exists" function
   *   pub fn table_exists(&self) -> Result<bool, Box<dyn Error>>
   */
  fn impl_sql_table_exists(&self) -> parse::Result<proc_macro2::TokenStream> {
    // let statement = format!("SELECT * FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_NAME = '{}'", self.get_table_name());
    let rusqlite_statement = format!("SELECT * FROM sqlite_master WHERE name='{}'", self.table_name());
    let prepare = sql::prepare(rusqlite_statement.as_str())?;
    Ok(quote::quote! {
      pub fn table_exists(&self) -> Result<bool, Box<dyn std::error::Error>> { Ok(#prepare.exists([])?) }
    })
  }

  /*
   * Output the implementation of
   *   pub fn delete_table(&self) -> Result<(), Box<dyn Error>>
   */
  fn impl_sql_delete_table(&self) -> parse::Result<proc_macro2::TokenStream> {
    let rusqlite_statement = format!("DROP TABLE {}", self.table_name()); 
    let execute = sql::execute(rusqlite_statement, Vec::new())?;
    Ok(quote::quote! {
      pub fn delete_table(&self) -> Result<(), Box<dyn std::error::Error>> { #execute; Ok(()) }
    })
  }

  /*
   * output the implementation the "create_table" function
   *   pub fn create_table(&self) -> Result<(), Box<dyn Error>>
   */
  fn impl_sql_create_table(&self) -> parse::Result<proc_macro2::TokenStream> {
    let fields_statement = Input::from(self.ast).fields()?.iter()
    .map(|f| Ok(format!("{} {}", f.ident_str()?, SqlType::from(f).to_string()) 
                + if f.attrs.primary_key { " PRIMARY_KEY"} else { "" }) )
    .collect::<parse::Result<Vec<String>>>()?.join(", ");
      
    let rusqlite_statement = format!("CREATE TABLE IF NOT EXISTS {} ( {} )", self.table_name(), fields_statement);
    let execute = sql::execute(rusqlite_statement, Vec::new())?;
    Ok(quote::quote! {
        pub fn create_table(&self) -> Result<(), Box<dyn std::error::Error>> { #execute; Ok(()) }
    })
  }

  fn impl_sql_constructor(&self) -> parse::Result<proc_macro2::TokenStream> {
    let input     = self.input();
    let class_sql = input.class_sql();
    let mut quote = quote::quote! { };

    let doc = format!(r#"
Construct a SQL connector to manipulate struct of type {} to an SQLite database using the rusqlite wrapper
"#, input.class());
    quote = quote::quote! {
       #quote
       #[doc = #doc]
       pub fn from_rusqlite(conn: &'a rusqlite::Connection) -> Result<#class_sql<'a>, Box<dyn std::error::Error>> {
         Ok( #class_sql { connection: sql::Connection::Rusqlite(conn) } )
       }
    };
    
    if let Some(f) = self.attrs.rusqlite_connection.as_ref().map(|f| f.value()) {
      let f_ident = syn::Ident::new(f.as_str(), proc_macro2::Span::call_site());
      let doc = format!(r#"
Construct a default SQL connector to manipulate struct of type {class_sql} to an SQLite database using the SQL connector creation function {f}
"#);
      quote = quote::quote! {
        #quote
        #[doc = #doc]
        pub fn from_database() -> Result<#class_sql<'a>, Box<dyn std::error::Error>> {
          Ok( #class_sql { connection: sql::Connection::RusqliteOwned(#f_ident()?) } )
        }
      };
    }

    Ok(quote)
  }
}

mod sql {
  use syn::parse;

  /// Output the generation of SQL connection wrapper
  pub fn impl_connection() -> parse::Result<proc_macro2::TokenStream> {
    let doc = "Provides different SQL connection wrapper supported";
    Ok(quote::quote! {
        #[doc = #doc]
        pub enum Connection<'a> {
          Rusqlite(&'a rusqlite::Connection),
          RusqliteOwned(rusqlite::Connection),
        }
    })
  }

  /// Output the code associated with a SQL prepare statement
  pub fn prepare(rusqlite_statement: &str) -> parse::Result<proc_macro2::TokenStream> {
    Ok(quote::quote! {
      match &self.connection {
        sql::Connection::Rusqlite(conn)        => conn.prepare(#rusqlite_statement)?,
        sql::Connection::RusqliteOwned(conn)   => conn.prepare(#rusqlite_statement)?,
      }
    })
  }

  /// Output the code associated with a SQL execute statement
  pub fn execute(rusqlite_statement: String, params: Vec<proc_macro2::TokenStream>) -> parse::Result<proc_macro2::TokenStream> {
    Ok(quote::quote! {
      match &self.connection {
        sql::Connection::Rusqlite(conn)        => conn.execute(#rusqlite_statement, ( #( #params ),* ))?,
        sql::Connection::RusqliteOwned(conn)   => conn.execute(#rusqlite_statement, ( #( #params ),* ))?,
      }
    })
  }
}
