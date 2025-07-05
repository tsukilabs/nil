// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod big_int;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(BigIntU64)]
pub fn derive_big_int_u64(input: TokenStream) -> TokenStream {
  let ast = syn::parse::<DeriveInput>(input).unwrap();
  big_int::impl_big_int_u64(&ast)
}
