// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]
#![feature(trim_prefix_suffix)]

mod building;
mod mine;
mod precursor;
mod report;
mod storage;
mod unit;

use proc_macro::TokenStream;

#[proc_macro_derive(Building)]
pub fn derive_building(input: TokenStream) -> TokenStream {
  building::impl_building(&syn::parse(input).unwrap())
}

#[proc_macro_derive(Mine)]
pub fn derive_mine(input: TokenStream) -> TokenStream {
  mine::impl_mine(&syn::parse(input).unwrap())
}

#[proc_macro_derive(Precursor)]
pub fn derive_precursor(input: TokenStream) -> TokenStream {
  precursor::impl_precursor(&syn::parse(input).unwrap())
}

#[proc_macro_derive(Report)]
pub fn derive_report(input: TokenStream) -> TokenStream {
  report::impl_report(&syn::parse(input).unwrap())
}

#[proc_macro_derive(Storage)]
pub fn derive_storage(input: TokenStream) -> TokenStream {
  storage::impl_storage(&syn::parse(input).unwrap())
}

#[proc_macro_derive(Unit)]
pub fn derive_unit(input: TokenStream) -> TokenStream {
  unit::impl_unit(&syn::parse(input).unwrap())
}
