mod core;

use proc_macro::TokenStream;

#[proc_macro_derive(Building)]
pub fn derive_building(input: TokenStream) -> TokenStream {
  core::impl_building(&syn::parse(input).unwrap())
}

#[proc_macro_derive(Unit)]
pub fn derive_unit(input: TokenStream) -> TokenStream {
  core::impl_unit(&syn::parse(input).unwrap())
}
