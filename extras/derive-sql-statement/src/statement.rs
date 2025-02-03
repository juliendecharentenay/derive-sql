use super::*;
use derive_sql_common::derive::fields;
use derive_sql_common::derive::SqlType;

pub struct Statement<'a> {
  ast: &'a syn::DeriveInput,
  fields_named: &'a syn::FieldsNamed,
}

impl<'a> TryFrom<&'a syn::DeriveInput> for Statement<'a> {
  type Error = syn::parse::Error;
  fn try_from(ast: &'a syn::DeriveInput) -> syn::parse::Result<Statement> {
    if let syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(fields_named), .. }) = &ast.data {
      Ok(Statement { ast, fields_named })
    } else {
      Err(syn::Error::new(ast.ident.span(), "Procedural macro DeriveStatement is intended to be applied to struct with named fields."))
    }
  }
}

impl<'a> Statement<'a> {
  pub fn generate(self) -> syn::parse::Result<proc_macro2::TokenStream> {
    let attrs = Attrs::from_attributes(&self.ast.attrs)?;
    let vis  = &self.ast.vis;
    let ident = &self.ast.ident;
    let sql_ident  = attrs.ident.as_ref().map(|i| i.clone()).unwrap_or_else(|| quote::format_ident!("Sql{ident}"));
    let table_name = attrs.table_name.as_ref().map(|i| i.clone()).unwrap_or_else(|| format!("{ident}").to_lowercase());

    let fields = self.fields_named.named.iter()
      .map(|f| f.try_into().map_err(|e| syn::Error::new(ident.span(), format!("{e}"))))
      .collect::<Result<Vec<fields::Fields>, syn::Error>>()?;

    // Primary key of SQL type TEXT is not supported
    if let Some(name) = fields.iter().fold(None, |r, f| r.or_else(|| if f.is_primary_key() && f.raw_type().eq("String") { Some(f.name()) } else { None })) {
      return Err(syn::Error::new(self.ast.ident.span(), format!("Field `{name}` error: Use of String, Option<String> primary key is not supported in `derive-sql` feature.")));
    }

    // List of token that convert column names with flavor
    let columns = fields.iter()
        .map(|f| {
          let ident = f.ident(); let s = format!("{ident}");
          Ok(quote::quote! { #ident = conn.flavor().column(#s)? })
        })
        .collect::<syn::parse::Result<Vec<proc_macro2::TokenStream>>>()?;
     let columns_types = fields.iter()
       .map(|f| {
         let ident = f.ident(); let ident_type = quote::format_ident!("{ident}_type");
         let ty = f.raw_type().to_string();
         Ok(quote::quote! { let #ident_type = conn.flavor().sql_type(#ty)?; })
       })
       .collect::<syn::parse::Result<Vec<proc_macro2::TokenStream>>>()?;

    let declaration = {
      let doc = format!("Wrapper struct to query item of type `{ident}` from SQL databases using `derive-sql` crate");
      quote::quote! {
        #[doc = #doc]
        #[derive(Default)]
        #vis struct #sql_ident {}
      }
    };

    let static_members = {
      let members = fields.iter().map(|f| f.as_pub_static_member()).collect::<Vec<proc_macro2::TokenStream>>();
      quote::quote! {
        pub const TABLE_NAME: &'static str = #table_name ;
        #( #members )*
      }
    };

    let table_statement = {
      let statement = format!("{}",
        {
          let mut a = fields.iter()
          .map(|f| Ok(format!("{{{ident}}} {{{ident}_type}}", ident = f.ident()))) //, sql_type = f.sql_type().to_string())) )
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

      let doc = format!("Create table `{table_name}` statement<br/>SQL statement:<br/>```CREATE TABLE {table_name} ( {statement} )```",
        statement = statement.replace("{","").replace("}",""));
      let create_stmt = quote::quote! {
        #[doc = #doc]
        fn create_stmt<C, R>(&self, conn: &C) -> derive_sql::Result<String>
        where C: derive_sql::traits::Connection<R>,
              R: derive_sql::traits::Row,
        {
          #(let #columns ; )*
          #(#columns_types)*
          Ok(format!("CREATE TABLE {table_name} ( {statement} )", 
            table_name = conn.flavor().table(#table_name)?, 
            statement = format!(#statement),
          ))
        }
      };

      let doc = format!("Create table if not exists `{table_name}` statement<br/>SQL statement:<br/>```CREATE TABLE IF NOT EXISTS {table_name} ( {statement} )```",
        statement = statement.replace("{","").replace("}",""));
      let create_if_not_exists_stmt = quote::quote! {
        #[doc = #doc]
        fn create_if_not_exist_stmt<C, R>(&self, conn: &C) -> derive_sql::Result<String> 
        where C: derive_sql::traits::Connection<R>,
              R: derive_sql::traits::Row,
        {
          #(let #columns ; )*
          #(#columns_types)*
          Ok(format!("CREATE TABLE IF NOT EXISTS {table_name} ( {statement} )", 
            table_name = conn.flavor().table(#table_name)?, 
            statement = format!(#statement),
          ))
        }
      };

      let doc = format!("Delete table if it exists `{table_name}` statement<br/>SQL statement:<br/>```DROP TABLE IF EXISTS {table_name}```");
      let drop_stmt = quote::quote! {
        #[doc = #doc]
        fn drop_stmt<C, R>(&self, conn: &C) -> derive_sql::Result<String>
        where C: derive_sql::traits::Connection<R>,
              R: derive_sql::traits::Row,
        {
          Ok(format!("DROP TABLE IF EXISTS {table_name}", 
            table_name = conn.flavor().table(#table_name)?))
        }
      };

      quote::quote! {
        impl derive_sql::traits::TableFlavoredStatement for #sql_ident {
          #create_stmt
          #create_if_not_exists_stmt
          #drop_stmt
        }
      }
    };

    let select_statement = {
      let statement = fields.iter()
        .map(|f| format!("{{{ident}}}", ident = f.ident())).collect::<Vec<String>>()
        .join(", ");
      let doc = format!("Statement to retrieve a list of `{ident}` items from database table `{table_name}`.<br/>SQL statement:<br/>```{statement}```",
        statement = statement.replace("{","").replace("}",""),
      );
      quote::quote! {
        impl derive_sql::traits::SelectFlavoredStatement for #sql_ident {
          #[doc = #doc]
          fn select_stmt<C, R>(&self, conn: &C) -> derive_sql::Result<String> 
          where C: derive_sql::traits::Connection<R>,
                R: derive_sql::traits::Row,
          {
            Ok(format!("SELECT {statement} FROM {table_name}", 
              table_name = conn.flavor().table(#table_name)?,
              statement = format!(#statement, #(#columns, )*),
            ))
          }
        }
      }
    };

    let insert_statement = {
      let columns_stmt = fields.iter().map(|f| format!("{{{ident}}}",ident = f.ident())).collect::<Vec<String>>().join(", ");
      let values_stmt = fields.iter().map(|_| format!("?")).collect::<Vec<String>>().join(", ");
      let values = fields.iter().enumerate().map(|(i,_)| quote::quote! { conn.flavor().value(#i)? }).collect::<Vec<proc_macro2::TokenStream>>();
      /*
      let statement = format!("INSERT INTO {table_name} ({}) VALUES ({})",
        fields.iter().map(|f| format!("`{}`",f.name())).collect::<Vec<String>>().join(", "),
        fields.iter().map(|_| format!("?")).collect::<Vec<String>>().join(", "),
      );
      */
      let doc = format!("Insert an item {ident} into the database table {table_name}<br/>SQL statement:<br/>```INSERT INTO {table_name} ({columns}) VALUES ({values})```",
        columns = columns_stmt.replace("{","").replace("}",""),
        values  = values_stmt,
      );
      quote::quote! {
        impl derive_sql::traits::InsertFlavoredStatement for #sql_ident {
          #[doc = #doc]
          fn insert_stmt<C, R>(&self, conn: &C) -> derive_sql::Result<String> 
          where C: derive_sql::traits::Connection<R>,
                R: derive_sql::traits::Row,
          {
            Ok(format!("INSERT INTO {table_name} ({columns}) VALUES ({values})", 
              table_name = conn.flavor().table(#table_name)?,
              columns = format!(#columns_stmt, #(#columns, )*),
              values = vec![#(#values, )*].join(", "),
            ))
          }
        }
      }
    };

    let update_statement = {
      let statement = fields.iter().map(|f| format!("{{{ident}}} = {{{ident}_value}}",ident = f.ident())).collect::<Vec<String>>().join(", ");
      let values  = fields.iter().enumerate()
        .map(|(i,f)| {
          let ident = quote::format_ident!("{ident}_value", ident = f.ident());
          quote::quote! { #ident = conn.flavor().value(#i)? }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();
      let doc = format!("Update item(s) nominated by the selector in the table {table_name}<br/>SQL statement:<br/>```UPDATE {table_name} SET {statement}```",
        statement = fields.iter().map(|f| format!("{ident} = ?", ident = f.ident())).collect::<Vec<String>>().join(", "));

      quote::quote! {
        impl derive_sql::traits::UpdateFlavoredStatement for #sql_ident {
          #[doc = #doc]
          fn update_stmt<C, R>(&self, conn: &C) -> derive_sql::Result<String> 
          where C: derive_sql::traits::Connection<R>,
                R: derive_sql::traits::Row
          {
            #(let #values ; )*
            #(let #columns ; )*
            Ok(format!("UPDATE {table_name} SET {statement}", 
              table_name = conn.flavor().table(#table_name)?,
              statement = format!(#statement),
            ))
          }
        }
      }
    };

    let delete_statement = {
      let statement = format!("DELETE FROM {table_name}");
      let doc = format!("Implementation of functionality to delete item(s) from database table `{table_name}`<br/>SQL statement:<br/>```{statement}```");
      quote::quote! {
        impl derive_sql::traits::DeleteStatement for #sql_ident {
          #[doc = #doc]
          fn delete_stmt(&self) -> derive_sql::Result<String> {
            Ok(format!("{}", #statement))
          }
        }
      }
    };

    let to_params = {
      let items = fields.iter()
      .map(|f| {
        let field = f.ident();
        quote::quote! { self.#field.to_param()? }
      })
      .collect::<Vec<proc_macro2::TokenStream>>();
      quote::quote! {
        impl derive_sql::traits::Params for #ident {
          fn as_vec_params(&self) -> derive_sql::Result<Vec<derive_sql::traits::Param>> {
            use derive_sql::traits::ToParam;
            Ok(
              vec![
                #( #items ),*
              ]
            )
          }
        }
      }
    };

    let try_from_ref_row = {
      let fields_assignment = fields.iter().enumerate()
      .map(|(i, f)| {
        let ident = f.ident();
        quote::quote! { #ident: r.get(#i).ok_or(derive_sql::Error::RowItemNotFound(#i))??  }
      })
      .collect::<Vec<proc_macro2::TokenStream>>();
      quote::quote! {
        impl<R> derive_sql::traits::TryFromRefRow<R> for #ident
        where R: derive_sql::traits::Row
        {
          fn try_from(r: &R) -> derive_sql::Result<Self> {
            Ok(
              #ident {
                #( #fields_assignment ),*
              }
            )
          }
        }
      }
    };

    let quote = if attrs.read_only {
      quote::quote! { 
        #try_from_ref_row
        #declaration
        impl #sql_ident { #static_members }
        #select_statement
      }

    } else {
      quote::quote! { 
        #to_params
        #try_from_ref_row
        #declaration
        impl #sql_ident { #static_members }
        #table_statement
        #select_statement
        #insert_statement
        #update_statement
        #delete_statement
      }
    };
    Ok(quote)
  }
}

