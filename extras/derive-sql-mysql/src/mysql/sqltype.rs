//! Define data type and their conversion to SQL data type
//!

pub enum SqlType {
  Integer,
  Text,
  Boolean,
  Float,
  DateTime,
  Date,
  Unsupported,
}

impl From<&syn::Field> for SqlType {
  fn from(f: &syn::Field) -> SqlType {
    (&f.ty).into()
  }
}

impl From<&syn::Type> for SqlType {
  fn from(t: &syn::Type) -> SqlType {
    SqlType::from_type(t)
  }
}

impl SqlType {
  pub fn from_type(ty: &syn::Type) -> SqlType {
    match ty {
      syn::Type::Path(syn::TypePath { path, .. }) if path.is_ident("String") => SqlType::Text,
      syn::Type::Path(syn::TypePath { path, .. }) if path.is_ident("i32")    => SqlType::Integer,
      syn::Type::Path(syn::TypePath { path, .. }) if path.is_ident("u32")    => SqlType::Integer,
      syn::Type::Path(syn::TypePath { path, .. }) if path.is_ident("usize")  => SqlType::Integer,
      syn::Type::Path(syn::TypePath { path, .. }) if path.is_ident("bool")   => SqlType::Boolean,
      syn::Type::Path(syn::TypePath { path, .. }) if path.is_ident("f32")    => SqlType::Float,
      syn::Type::Path(syn::TypePath { path: syn::Path { segments, .. } , .. }) 
      if segments.last().and_then(|p| Some(p.ident == "DateTime")).unwrap_or(false) => SqlType::DateTime,
      syn::Type::Path(syn::TypePath { path: syn::Path { segments, .. } , .. }) 
      if segments.last().and_then(|p| Some(p.ident == "NaiveDate")).unwrap_or(false) => SqlType::Date,
      _ => SqlType::Unsupported,
    }
  }

  pub fn to_string(&self) -> &str {
    match self {
      SqlType::Integer     => "INTEGER",
      SqlType::Text        => "TEXT",
      SqlType::Boolean     => "BOOL",
      SqlType::Float       => "FLOAT",
      SqlType::DateTime    => "DATETIME",
      SqlType::Date        => "DATE",
      SqlType::Unsupported => "", 
    }
  }
}

#[cfg(test)]
mod test_sql_type {
  use super::*;
  
  #[test]
  fn works() -> Result<(), Box<dyn std::error::Error>> {
    let t = syn::parse_str::<syn::Type>("[u32; 6]")?;
    let t = SqlType::from_type(&t);
    assert!(matches!(t, SqlType::Unsupported));
    assert!(t.to_string().eq(""));
    
    for k in ["u32", "usize", "i32"] {
      let t = syn::parse_str::<syn::Type>(k)?;
      let t = SqlType::from_type(&t);
      assert!(matches!(t, SqlType::Integer));
      assert!(t.to_string().eq("INTEGER"));
    }
    
    let t = syn::parse_str::<syn::Type>("String")?;
    let t = SqlType::from_type(&t);
    assert!(matches!(t, SqlType::Text));
    assert!(t.to_string().eq("TEXT"));
    
    let t = syn::parse_str::<syn::Type>("bool")?;
    let t = SqlType::from_type(&t);
    assert!(matches!(t, SqlType::Boolean));
    assert!(t.to_string().eq("BIT"));

    let t = syn::parse_str::<syn::Type>("f32")?;
    let t = SqlType::from_type(&t);
    assert!(matches!(t, SqlType::Float));
    assert!(t.to_string().eq("FLOAT"));
    
    let t = syn::parse_str::<syn::Type>("DateTime")?;
    let t = SqlType::from_type(&t);
    assert!(matches!(t, SqlType::DateTime));
    assert!(t.to_string().eq("DATETIME"));

    let t = syn::parse_str::<syn::Type>("NaiveDate")?;
    let t = SqlType::from_type(&t);
    assert!(matches!(t, SqlType::Date));
    assert!(t.to_string().eq("DATE"));

    Ok(())
  }
}
