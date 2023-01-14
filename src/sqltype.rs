//! Define data type and their conversion to SQL data type
//!

pub enum SqlType {
  Integer,
  Text,
  Boolean,
  DateTime,
  Unsupported,
}

impl SqlType {
  pub fn from_type(ty: &syn::Type) -> SqlType {
    match ty {
      syn::Type::Path(syn::TypePath { path, .. }) if path.is_ident("String") => SqlType::Text,
      syn::Type::Path(syn::TypePath { path, .. }) if path.is_ident("u32")    => SqlType::Integer,
      syn::Type::Path(syn::TypePath { path, .. }) if path.is_ident("bool")   => SqlType::Boolean,
      syn::Type::Path(syn::TypePath { path: syn::Path { segments, .. } , .. }) 
      if segments.last().and_then(|p| Some(p.ident == "DateTime")).unwrap_or(false) => SqlType::DateTime,
      _ => SqlType::Unsupported,
    }
  }

  pub fn to_string(&self) -> &str {
    match self {
      SqlType::Integer     => "INTEGER",
      SqlType::Text        => "TEXT",
      SqlType::Boolean     => "BIT",
      SqlType::DateTime    => "DATETIME",
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
    
    let t = syn::parse_str::<syn::Type>("u32")?;
    let t = SqlType::from_type(&t);
    assert!(matches!(t, SqlType::Integer));
    assert!(t.to_string().eq("INTEGER"));
    
    let t = syn::parse_str::<syn::Type>("String")?;
    let t = SqlType::from_type(&t);
    assert!(matches!(t, SqlType::Text));
    assert!(t.to_string().eq("TEXT"));
    
    let t = syn::parse_str::<syn::Type>("bool")?;
    let t = SqlType::from_type(&t);
    assert!(matches!(t, SqlType::Boolean));
    assert!(t.to_string().eq("BIT"));
    
    let t = syn::parse_str::<syn::Type>("DateTime")?;
    let t = SqlType::from_type(&t);
    assert!(matches!(t, SqlType::DateTime));
    assert!(t.to_string().eq("DATETIME"));

    Ok(())
  }
}
