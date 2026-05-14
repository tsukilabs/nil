// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::get_tuple_struct_inner_ident;
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_const_deref(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let inner = get_tuple_struct_inner_ident(ast);

  let stream = quote! {
    #[automatically_derived]
    impl const ::core::ops::Deref for #name {
      type Target = #inner;

      fn deref(&self) -> &Self::Target {
        &self.0
      }
    }
  };

  stream.into()
}
