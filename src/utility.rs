pub fn get_fields_named(ast: &syn::DeriveInput) -> Option<&syn::FieldsNamed> {
    if let Some(syn::Fields::Named(fields_named)) = get_fields(ast) {
      Some(fields_named)
    } else {
      None
    }
}

pub fn get_fields(ast: &syn::DeriveInput) -> Option<&syn::Fields> {
    if let syn::Data::Struct(syn::DataStruct { fields, .. }) = &ast.data {
      Some(fields)
    } else {
      None
    }
}
