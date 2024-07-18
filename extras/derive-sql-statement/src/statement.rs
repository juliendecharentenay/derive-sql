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
    if let Some(name) = fields.iter().fold(None, |r, f| r.or_else(|| if f.is_primary_key() && (matches!(f.sql_type(), SqlType::Text) || matches!(f.sql_type(), SqlType::OptionText)) { Some(f.name()) } else { None })) {
      return Err(syn::Error::new(self.ast.ident.span(), format!("Field `{name}` error: Use of String, Option<String> primary key is not supported in `derive-sql` feature.")));
    }


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
          .map(|f| Ok(format!("`{ident}` {sql_type}", ident = f.ident(), sql_type = f.sql_type().to_string())) )
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
      let doc = format!("Create table `{table_name}` statement<br/>SQL statement:<br/>```CREATE TABLE {table_name} ( {statement} )```");
      let create_stmt = quote::quote! {
        #[doc = #doc]
        fn create_stmt(&self) -> derive_sql::Result<String> {
          Ok(format!("CREATE TABLE {table_name} ( {statement} )", table_name = #table_name, statement = #statement))
        }
      };

      let doc = format!("Create table if not exists `{table_name}` statement<br/>SQL statement:<br/>```CREATE TABLE IF NOT EXISTS {table_name} ( {statement} )```");
      let create_if_not_exists_stmt = quote::quote! {
        #[doc = #doc]
        fn create_if_not_exist_stmt(&self) -> derive_sql::Result<String> {
          Ok(format!("CREATE TABLE IF NOT EXISTS {table_name} ( {statement} )", table_name = #table_name, statement = #statement))
        }
      };

      let doc = format!("Delete table if it exists `{table_name}` statement<br/>SQL statement:<br/>```DROP TABLE IF EXISTS {table_name}```");
      let drop_stmt = quote::quote! {
        #[doc = #doc]
        fn drop_stmt(&self) -> derive_sql::Result<String> {
          Ok(format!("DROP TABLE IF EXISTS {table_name}", table_name = #table_name))
        }
      };

      quote::quote! {
        impl derive_sql::traits::TableStatement for #sql_ident {
          #create_stmt
          #create_if_not_exists_stmt
          #drop_stmt
        }
      }
    };

    let select_statement = {
      let statement = format!("SELECT {} FROM {table_name}",
        fields.iter().map(|f| format!("`{}`",f.name())).collect::<Vec<String>>().join(", ")
      );
      let doc = format!("Statement to retrieve a list of `{ident}` items from database table `{table_name}`.<br/>SQL statement:<br/>```{statement}```");
      quote::quote! {
        impl derive_sql::traits::SelectStatement for #sql_ident {
          #[doc = #doc]
          fn select_stmt(&self) -> derive_sql::Result<String> {
            Ok(format!("{}", #statement))
          }
        }
      }
    };

    let insert_statement = {
      let statement = format!("INSERT INTO {table_name} ({}) VALUES ({})",
        fields.iter().map(|f| format!("`{}`",f.name())).collect::<Vec<String>>().join(", "),
        fields.iter().map(|_| format!("?")).collect::<Vec<String>>().join(", "),
      );
      let doc = format!("Insert an item {ident} into the database table {table_name}<br/>SQL statement:<br/>```{statement}```");
      quote::quote! {
        impl derive_sql::traits::InsertStatement for #sql_ident {
          #[doc = #doc]
          fn insert_stmt(&self) -> derive_sql::Result<String> {
            Ok(format!("{}", #statement))
          }
        }
      }
    };

    let update_statement = {
      let statement = format!("UPDATE {table_name} SET {}",
        fields.iter()
        .map(|f| format!("`{}` = ?", f.ident()))
        .collect::<Vec<String>>().join(", ")
      );
      let doc = format!("Update item(s) nominated by the selector in the table {table_name}<br/>SQL statement:<br/>```{statement}```");

      quote::quote! {
        impl derive_sql::traits::UpdateStatement for #sql_ident {
          #[doc = #doc]
          fn update_stmt(&self) -> derive_sql::Result<String> {
            Ok(format!("{}", #statement))
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

