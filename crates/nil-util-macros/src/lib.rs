// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]

mod r#impl;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(BigInt)]
pub fn derive_big_int(input: TokenStream) -> TokenStream {
  let ast = syn::parse::<DeriveInput>(input).unwrap();
  r#impl::big_int::impl_big_int(&ast)
}

#[proc_macro_derive(ConstDeref)]
pub fn derive_const_deref(input: TokenStream) -> TokenStream {
  let ast = syn::parse::<DeriveInput>(input).unwrap();
  r#impl::deref::impl_const_deref(&ast)
}

#[proc_macro_derive(F64Math)]
pub fn derive_f64_math(input: TokenStream) -> TokenStream {
  let ast = syn::parse::<DeriveInput>(input).unwrap();
  r#impl::f64::impl_f64_math(&ast)
}
