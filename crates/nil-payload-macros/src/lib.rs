// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]

mod response;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(IntoJsonResponse)]
pub fn derive_into_json_response(input: TokenStream) -> TokenStream {
  let ast = syn::parse::<DeriveInput>(input).unwrap();
  response::impl_into_json_response(&ast)
}
