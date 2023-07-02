//! Crate to convert derive macro helper attributes into a struct.
//!
//! More information on derive macro helper attributes
//! at <https://doc.rust-lang.org/reference/procedural-macros.html#derive-macro-helper-attributes>
//!
//! Parse a helper attributes of the for:
//! ```ignore
//! #[tea(kind = "EarlGrey", hot)]
//! struct MyStruct {
//! }
//! ```
//!
//! Into a struct defined as:
//!
//! ```
//! use from_attributes::DeriveFromAttributes;
//! 
//! #[derive(DeriveFromAttributes)]
//! #[from_attributes(ident = "tea")]
//! struct TeaAttr {
//!   kind: Option<syn::LitStr>,
//!   hot: bool,
//! }
//! ```
//!
//! Using, in the derive macro:
//!
//! ```text
//! #[proc_macro_derive(Drink, attribute(tea)]
//! pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//!   syn::parse(input)
//!   .and_then(|ast: syn::DeriveInput| {
//!     let tea_attr: TeaAttr = (&ast.attrs).try_into()?;
//!     Ok(quote::quote! { } )
//!   })
//!   .unwrap_or_else(|e| e.into_compile_error().into())
//! }
//! ```
//!
//! In all likelyhood, the crate `darling` includes the above functionality.
//!

mod derive;

#[proc_macro_derive(DeriveFromAttributes, attributes(from_attributes))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  syn::parse(input)
  .and_then(|ast: syn::DeriveInput| {
    derive::Derive::new(ast).generate()
  })
  .and_then(|ts| Ok(ts.into()))
  .unwrap_or_else(|e| e.into_compile_error().into())
}

