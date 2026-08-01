// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::get_tuple_struct_inner_ident;
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_clamp_new(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let inner = get_tuple_struct_inner_ident(ast);
  let inner = inner.to_string();

  let new_arg = if inner.starts_with("NonZero") {
    quote! { self.0.get() }
  } else {
    quote! { self.0 }
  };

  let stream = quote! {
    #[automatically_derived]
    impl #name {
      #[inline]
      pub const fn clamp(&mut self) {
        *self = Self::new(#new_arg);
      }
    }
  };

  stream.into()
}
