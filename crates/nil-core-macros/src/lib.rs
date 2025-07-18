// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod infrastructure;
mod npc;
mod unit;

use proc_macro::TokenStream;

#[proc_macro_derive(Building)]
pub fn derive_building(input: TokenStream) -> TokenStream {
  infrastructure::impl_building(&syn::parse(input).unwrap())
}

#[proc_macro_derive(Mine)]
pub fn derive_mine(input: TokenStream) -> TokenStream {
  infrastructure::impl_mine(&syn::parse(input).unwrap())
}

#[proc_macro_derive(Precursor)]
pub fn derive_precursor(input: TokenStream) -> TokenStream {
  npc::impl_precursor(&syn::parse(input).unwrap())
}

#[proc_macro_derive(Storage)]
pub fn derive_storage(input: TokenStream) -> TokenStream {
  infrastructure::impl_storage(&syn::parse(input).unwrap())
}

#[proc_macro_derive(Unit)]
pub fn derive_unit(input: TokenStream) -> TokenStream {
  unit::impl_unit(&syn::parse(input).unwrap())
}
