use super::*;
use derive_sql_common::derive::fields;
use derive_sql_common::derive::SqlType;

pub struct Mysql<'a> {
  ast: &'a syn::DeriveInput,
  fields_named: &'a syn::FieldsNamed,
}

impl<'a> TryFrom<&'a syn::DeriveInput> for Mysql<'a> {
  type Error = syn::parse::Error;
  fn try_from(ast: &'a syn::DeriveInput) -> syn::parse::Result<Mysql> {
    if let syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(fields_named), .. }) = &ast.data {
      Ok(Mysql { ast, fields_named })
    } else {
      Err(syn::Error::new(ast.ident.span(), "Procedural macro DeriveMysql is intended to be applied to struct with named fields."))
    }
  }
}

impl<'a> Mysql<'a> {
  pub fn generate(self) -> syn::parse::Result<proc_macro2::TokenStream> {
    let attrs = Attrs::from_attributes(&self.ast.attrs)?;
    let vis  = &self.ast.vis;
    let ident = &self.ast.ident;
    let mysql_ident  = attrs.ident.as_ref().map(|i| i.clone()).unwrap_or_else(|| quote::format_ident!("{ident}Mysql"));
    let table_name   = attrs.table_name.as_ref().map(|i| i.clone()).unwrap_or_else(|| format!("{ident}").to_lowercase());

    let fields = self.fields_named.named.iter()
      .map(|f| f.try_into().map_err(|e| syn::Error::new(ident.span(), format!("{e}"))))
      .collect::<Result<Vec<fields::Fields>, syn::Error>>()?;

    // Primary key of SQL type TEXT is not supported
    if let Some(name) = fields.iter().fold(None, |r, f| r.or_else(|| if f.is_primary_key() && matches!(f.sql_type(), SqlType::Text) { Some(f.name()) } else { None })) {
      return Err(syn::Error::new(self.ast.ident.span(), format!("Field `{name}` error: Use of String/TEXT primary key is not supported in `derive-sql` feature `MySQL`.")));
    }


    let declaration = {
      let doc = format!("Wrapper struct to query item of type `{ident}` from MySQL database using `mysql` crate");
      quote::quote! {
        #[doc = #doc]
        #vis struct #mysql_ident<T>
        where T: mysql::prelude::Queryable
        { 
          conn: std::cell::RefCell<T>,
        }
      }
    };

    let from_mysql_impl = {
      let doc = format!("Create a new instance from a `mysql` connection");
      quote::quote! {
        impl<T> std::convert::From<T> for #mysql_ident<T>
        where T: mysql::prelude::Queryable
        {
          #[doc = #doc]
          fn from(conn: T) -> Self { #mysql_ident { conn: std::cell::RefCell::new(conn) } }
        }
      }
    };

    let create_table = {
      let doc = format!("Create table `{table_name}` in the MySQL database.");
      let statement = format!("CREATE TABLE IF NOT EXISTS {table_name} ( {} )",
        {
          let mut a = fields.iter()
          .map(|f| {
            let ident = f.ident();
            let sql_type = f.sql_type().to_string();
            Ok(format!("`{ident}` {sql_type}"))
          })
          .collect::<syn::parse::Result<Vec<String>>>()?;
          if let Some(primary_key) = fields.iter().fold(None, |r, f| r.or_else(|| if f.is_primary_key() { Some(f) } else { None })) {
            a.push(format!("PRIMARY KEY ( `{}` )", primary_key.ident()));
          }
          for f in fields.iter().filter(|f| f.is_unique()) {
            a.push(format!("CONSTRAINT {0}_unique UNIQUE ( `{0}` )", f.ident()));
          }
          a.join(", ")
        }
      );
      let doc = format!("{doc}<br>SQL statement: `{statement}`");
      quote::quote! {
        #[doc = #doc]
        pub fn create_table(&mut self) -> Result<(), Box<dyn std::error::Error>> {
          use mysql::prelude::Queryable;
          let stmt = format!("{}", #statement);
          self.conn.borrow_mut().query_drop(stmt.as_str())?;
          Ok(())
        }
      }
    };

    let count = {
      let doc = format!("Implementation of functionality to count the number of item(s) from database table `{table_name}`");
      let statement = format!("SELECT COUNT(*) FROM {table_name}");
      let doc = format!("{doc}<br>SQL statement: `{statement}`");
      quote::quote! {
        #[doc = #doc]
        fn count(&self, select: Self::Selector) -> Result<usize, Self::Error> {
          use mysql::prelude::Queryable;
          let stmt = format!("{} {}", #statement, select.statement());
          let r = self.conn.borrow_mut().query_first(stmt.as_str())?.unwrap_or(0);
          Ok(r)
        }
      }
    };

    let select = {
      let doc = format!("Retrieve a list of `{ident}` items matching the selector parameter from database table `{table_name}`");
      let statement = format!("SELECT {} FROM {table_name}",
        fields.iter().map(|f| format!("`{}`",f.name())).collect::<Vec<String>>().join(", ")
      );
      let doc = format!("{doc}<br>SQL statement: `{statement}`");
      let assignements = fields.iter().enumerate()
        .map(|(i, f)| {
          let name = f.name();
          // quote::quote! { r.get(#i).ok_or(format!("Unable to retrieve field {}", #name))? }
          quote::quote! { r.get_opt(#i)
            .ok_or(format!("Unable to retrieve field {}", #name))?
            .map_err(|e| format!("field `{}` error: {e}", #name))? 
            } 
        })
        .collect::<Vec<proc_macro2::TokenStream>>();
      let fields = fields.iter().map(|f| f.ident()).collect::<Vec<&syn::Ident>>();
      quote::quote! {
        #[doc = #doc]
        fn select(&self, select: Self::Selector) -> Result<Vec<Self::Item>, Self::Error> {
          use mysql::prelude::Queryable;
          let stmt = format!("{} {}", #statement, select.statement());
          let r = self.conn.borrow_mut().query_map(stmt.as_str(), 
            |r: mysql::Row| Ok( #ident { #( #fields: #assignements ),* } )
          )?
          .into_iter()
          .collect::<Result<Vec<Self::Item>, String>>()?;
          Ok(r)
        }
      }
    };

    let insert = {
      let doc = format!("Insert an item {ident} into the database table {table_name}");
      let functions = fields.iter()
        .filter_map(|f| {
          let ident = f.ident();
          f.on_insert().as_ref().map(|p| quote::quote! { item.#ident = #p(); })
        })
        .collect::<Vec<proc_macro2::TokenStream>>();
      let statement = format!("INSERT INTO {table_name} ({}) VALUES ({})",
        fields.iter().map(|f| format!("`{}`",f.name())).collect::<Vec<String>>().join(", "),
        fields.iter().map(|f| format!(":{}", f.name())).collect::<Vec<String>>().join(", "),
      );
      let doc = format!("{doc}<br>SQL statement: `{statement}`");
      let names  = fields.iter().map(|f| f.name()).collect::<Vec<String>>();
      let params = fields.iter().map(|f| f.ident()).collect::<Vec<&syn::Ident>>();
      quote::quote! {
        #[doc = #doc]
        fn insert(&mut self, mut item: Self::Item) -> Result<Self::Item, Self::Error> {
          use mysql::{params, prelude::Queryable};
          #( #functions )*
          let stmt = format!("{}", #statement);
          self.conn.borrow_mut().exec_drop(stmt.as_str(), 
            mysql::params! {
              #( #names => &item.#params ), *
            }
          )?;
          Ok(item)
        }
      }
    };

    let update = {
      let doc = format!("Update item(s) nominated by the selector in the table {table_name}");
      let functions = fields.iter()
        .filter_map(|f| {
          let ident = f.ident();
          f.on_update().as_ref().map(|p| quote::quote! { item.#ident = #p(); })
        })
        .collect::<Vec<proc_macro2::TokenStream>>();
      let statement = format!("UPDATE {table_name} SET {}",
        fields.iter()
        .map(|f| format!("`{}` = :{}", f.ident(), f.name()))
        .collect::<Vec<String>>().join(", ")
      );
      let doc = format!("{doc}<br>SQL statement: `{statement}`");
      let names  = fields.iter().map(|f| f.name()).collect::<Vec<String>>();
      let params = fields.iter().map(|f| f.ident()).collect::<Vec<&syn::Ident>>();

      quote::quote! {
        #[doc = #doc]
        fn update(&mut self, select: Self::Selector, mut item: Self::Item) -> Result<Self::Item, Self::Error> {
          use mysql::{params, prelude::Queryable};
          #( #functions )*
          let stmt = format!("{} {}", #statement, select.statement());
          self.conn.borrow_mut().exec_drop(stmt.as_str(), 
            mysql::params! {
              #( #names => &item.#params ), *
            }
          )?;
          Ok(item)
        }
      }
    };

    let delete = {
      let doc = format!("Implementation of functionality to delete item(s) from database table `{table_name}`");
      let statement = format!("DELETE FROM {table_name}");
      let doc = format!("{doc}<br>SQL statement: `{statement}`");
      quote::quote! {
        #[doc = #doc]
        fn delete(&mut self, select: Self::Selector) -> Result<(), Self::Error> {
          use mysql::prelude::Queryable;
          let stmt = format!("{} {}", #statement, select.statement());
          self.conn.borrow_mut().exec_drop(stmt.as_str(), ())?;
          Ok(())
        }
      }
    };

    let delete_table = {
      let doc = format!("Delete table `{table_name}` from SQLite database");
      let statement = format!("DROP TABLE {table_name}");
      let doc = format!("{doc}<br>SQL statement: `{statement}`");
      quote::quote! {
        #[doc = #doc]
        fn delete_table(&mut self) -> Result<(), Self::Error> {
          use mysql::prelude::Queryable;
          self.conn.borrow_mut().query_drop(#statement)?;
          Ok(())
        }
      }
    };

    Ok(quote::quote! { 
      #declaration
      #from_mysql_impl

      impl<T> #mysql_ident<T>
      where T: mysql::prelude::Queryable 
      {
        #create_table
      }

      impl<T> derive_sql::Sqlable for #mysql_ident<T>
      where T: mysql::prelude::Queryable
      {
        type Item = #ident;
        type Error = Box<dyn std::error::Error>;
        type Selector = Box<dyn derive_sql::Selectable>;
        #count
        #select
        #insert
        #update
        #delete
        #delete_table
      }
    })
  }
}

