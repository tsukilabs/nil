mod building;
mod unit;

use proc_macro::TokenStream;

#[proc_macro_derive(Building)]
pub fn derive_building(input: TokenStream) -> TokenStream {
  building::impl_building(&syn::parse(input).unwrap())
}

#[proc_macro_derive(Unit)]
pub fn derive_unit(input: TokenStream) -> TokenStream {
  unit::impl_unit(&syn::parse(input).unwrap())
}
