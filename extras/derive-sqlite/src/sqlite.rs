use super::*;

mod fields;
mod sqltype; pub use sqltype::SqlType;

pub struct Sqlite<'a> {
  ast: &'a syn::DeriveInput,
  fields_named: &'a syn::FieldsNamed,
}

impl<'a> TryFrom<&'a syn::DeriveInput> for Sqlite<'a> {
  type Error = syn::parse::Error;
  fn try_from(ast: &'a syn::DeriveInput) -> syn::parse::Result<Sqlite> {
    if let syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(fields_named), .. }) = &ast.data {
      Ok(Sqlite { ast, fields_named })
    } else {
      Err(syn::Error::new(ast.ident.span(), "Procedural macro DeriveSqlite is intended to be applied to struct with named fields."))
    }
  }
}

impl<'a> Sqlite<'a> {
  pub fn generate(self) -> syn::parse::Result<proc_macro2::TokenStream> {
    let attrs = Attrs::from_attributes(&self.ast.attrs)?;
    let vis  = &self.ast.vis;
    let ident = &self.ast.ident;
    let sqlite_ident = attrs.ident.as_ref().map(|i| i.clone()).unwrap_or_else(|| quote::format_ident!("{ident}Sqlite"));
    let table_name   = attrs.table_name.as_ref().map(|i| i.clone()).unwrap_or_else(|| format!("{ident}").to_lowercase());

    let fields = self.fields_named.named.iter()
      .map(|f| f.try_into().map_err(|e| syn::Error::new(ident.span(), format!("{e}"))))
      .collect::<Result<Vec<fields::Fields>, syn::Error>>()?;

    let declaration = {
      let doc = format!("Wrapper struct to query item of type `{ident}` from SQLite database using `rusqlite` library");
      quote::quote! {
        #[doc = #doc]
        #vis struct #sqlite_ident { 
          conn: rusqlite::Connection,
        }
      }
    };

    let from_rusqlite_impl = {
      let doc = format!("Create a new instance from a `rusqlite` connection");
      quote::quote! {
        impl std::convert::From<rusqlite::Connection> for #sqlite_ident {
          #[doc = #doc]
          fn from(v: rusqlite::Connection) -> Self { #sqlite_ident { conn: v } }
        }
      }
    };

    let create_table = {
      let doc = format!("Create table `{table_name}` in the SQLite database");
      let statement = format!("CREATE TABLE IF NOT EXISTS {table_name} ( {} )",
        fields.iter()
        .map(|f| {
          let ident = f.ident();
          let sql_type = f.sql_type().to_string();
          if f.is_primary_key() {
            Ok(format!("{ident} {sql_type} PRIMARY KEY"))
          } else {
            Ok(format!("{ident} {sql_type}"))
          }
        })
        .collect::<syn::parse::Result<Vec<String>>>()?
        .join(", ")
      );
      quote::quote! {
        #[doc = #doc]
        fn create_table(&mut self) -> Result<(), Box<dyn std::error::Error>> {
          let stmt = format!("{}", #statement);
          self.conn.execute(stmt.as_str(), ())?;
          Ok(())
        }
      }
    };

    let count = {
      let doc = format!("Implementation of functionality to count the number of item(s) from database table `{table_name}`");
      let statement = format!("SELECT COUNT(*) FROM {table_name}");
      quote::quote! {
        #[doc = #doc]
        fn count(&self, select: Self::Selector) -> Result<usize, Self::Error> {
          let stmt = format!("{} {}", #statement, select.statement());
          let r = self.conn.prepare(stmt.as_str())?
          .query_row([], |r| r.get(0))?;
          Ok(r)
        }
      }
    };

    let select = {
      let doc = format!("Retrieve a list of `{ident}` items matching the selector parameter from SQLite database table `{table_name}`");
      let statement = format!("SELECT {} FROM {table_name}",
        fields.iter().map(|f| f.name()).collect::<Vec<String>>().join(", ")
      );
      let fields = fields.iter().map(|f| f.ident()).collect::<Vec<&syn::Ident>>();
      let assignements = fields.iter().enumerate().map(|(i, _)| quote::quote! { r.get(#i)? } ).collect::<Vec<proc_macro2::TokenStream>>();
      quote::quote! {
        #[doc = #doc]
        fn select(&self, select: Self::Selector) -> Result<Vec<Self::Item>, Self::Error> {
          let stmt = format!("{} {}", #statement, select.statement());
          let r = self.conn.prepare(stmt.as_str())?
          .query_map([], |r| Ok( #ident { #( #fields: #assignements ),* } ))?
          .collect::<Result<Vec<Self::Item>, rusqlite::Error>>()?;
          Ok(r)
        }
      }
    };

    let insert = {
      let doc = format!("Insert an item {ident} into the SQLite database table {table_name}");
      let functions = fields.iter()
        .filter_map(|f| {
          let ident = f.ident();
          f.on_insert().as_ref().map(|p| quote::quote! { item.#ident = #p(); })
        })
        .collect::<Vec<proc_macro2::TokenStream>>();
      let statement = format!("INSERT INTO {table_name} ({}) VALUES ({})",
        fields.iter().map(|f| f.name()).collect::<Vec<String>>().join(", "),
        fields.iter().enumerate().map(|(i,_)| format!("?{}", i+1)).collect::<Vec<String>>().join(", ")
      );
      let params = fields.iter().map(|f| f.ident()).collect::<Vec<&syn::Ident>>();
      quote::quote! {
        #[doc = #doc]
        fn insert(&mut self, mut item: Self::Item) -> Result<Self::Item, Self::Error> {
          #( #functions )*
          let stmt = format!("{}", #statement);
          self.conn.execute(stmt.as_str(), ( #( &item.#params ),* ))?;
          Ok(item)
        }
      }
    };

    let update = {
      let doc = format!("Update item(s) nominated by the selector in the SQLite table {table_name}");
      let functions = fields.iter()
        .filter_map(|f| {
          let ident = f.ident();
          f.on_update().as_ref().map(|p| quote::quote! { item.#ident = #p(); })
        })
        .collect::<Vec<proc_macro2::TokenStream>>();
      let statement = format!("UPDATE {table_name} SET {}",
        fields.iter().enumerate()
        .map(|(i,f)| format!("{} = ?{}", f.ident(), i+1))
        .collect::<Vec<String>>().join(", ")
      );
      let params = fields.iter().map(|f| f.ident()).collect::<Vec<&syn::Ident>>();

      quote::quote! {
        #[doc = #doc]
        fn update(&mut self, select: Self::Selector, mut item: Self::Item) -> Result<Self::Item, Self::Error> {
          #( #functions )*
          let stmt = format!("{} {}", #statement, select.statement());
          self.conn.execute(stmt.as_str(), ( #( &item.#params ),* ))?;
          Ok(item)
        }
      }
    };

    let delete = {
      let doc = format!("Implementation of functionality to delete item(s) from database table `{table_name}`");
      let statement = format!("DELETE FROM {table_name}");
      quote::quote! {
        #[doc = #doc]
        fn delete(&mut self, select: Self::Selector) -> Result<(), Self::Error> {
          let stmt = format!("{} {}", #statement, select.statement());
          self.conn.execute(stmt.as_str(), ())?;
          Ok(())
        }
      }
    };

    let delete_table = {
      let doc = format!("Delete table `{table_name}` from SQLite database");
      let statement = format!("DROP TABLE {table_name}");
      quote::quote! {
        #[doc = #doc]
        fn delete_table(&mut self) -> Result<(), Self::Error> {
          self.conn.execute(#statement, ())?;
          Ok(())
        }
      }
    };

    Ok(quote::quote! { 
      #declaration
      #from_rusqlite_impl

      impl #sqlite_ident {
        #create_table
      }

      impl derive_sql::Sqlable for #sqlite_ident {
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

