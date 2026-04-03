// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]

mod big_int;
mod ops;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(BigIntU64)]
pub fn derive_big_int_u64(input: TokenStream) -> TokenStream {
  let ast = syn::parse::<DeriveInput>(input).unwrap();
  big_int::impl_big_int_u64(&ast)
}

#[proc_macro_derive(BigIntUsize)]
pub fn derive_big_int_usize(input: TokenStream) -> TokenStream {
  let ast = syn::parse::<DeriveInput>(input).unwrap();
  big_int::impl_big_int_usize(&ast)
}

#[proc_macro_derive(F64Ops)]
pub fn derive_f64_ops(input: TokenStream) -> TokenStream {
  let ast = syn::parse::<DeriveInput>(input).unwrap();
  ops::impl_f64_ops(&ast)
}

#[proc_macro_derive(F64Add)]
pub fn derive_f64_add(input: TokenStream) -> TokenStream {
  let ast = syn::parse::<DeriveInput>(input).unwrap();
  ops::impl_f64_add(&ast)
}

#[proc_macro_derive(F64Sub)]
pub fn derive_f64_sub(input: TokenStream) -> TokenStream {
  let ast = syn::parse::<DeriveInput>(input).unwrap();
  ops::impl_f64_sub(&ast)
}

#[proc_macro_derive(F64Mul)]
pub fn derive_f64_mul(input: TokenStream) -> TokenStream {
  let ast = syn::parse::<DeriveInput>(input).unwrap();
  ops::impl_f64_mul(&ast)
}

#[proc_macro_derive(F64Div)]
pub fn derive_f64_div(input: TokenStream) -> TokenStream {
  let ast = syn::parse::<DeriveInput>(input).unwrap();
  ops::impl_f64_div(&ast)
}
