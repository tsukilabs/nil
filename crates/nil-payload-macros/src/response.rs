// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_into_json_response(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    #[automatically_derived]
    impl ::axum::response::IntoResponse for #name {
      fn into_response(self) -> ::axum::response::Response {
        ::axum::extract::Json(self).into_response()
      }
    }
  };

  stream.into()
}
