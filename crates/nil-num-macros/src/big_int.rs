// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::DeriveInput;

pub fn impl_big_int_u64(ast: &DeriveInput) -> TokenStream {
  impl_big_int(ast, &Ident::new("u64", Span::call_site()))
}

pub fn impl_big_int_usize(ast: &DeriveInput) -> TokenStream {
  impl_big_int(ast, &Ident::new("usize", Span::call_site()))
}

fn impl_big_int(ast: &DeriveInput, int: &Ident) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    #[automatically_derived]
    impl ::serde::Serialize for #name {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
        S: ::serde::Serializer,
      {
        serializer.serialize_str(self.0.to_string().as_str())
      }
    }

    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for #name {
      fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where
        D: ::serde::Deserializer<'de>,
      {
        use ::serde::de::Error as _;
        String::deserialize(deserializer)?
          .parse::<#int>()
          .map(Self)
          .map_err(D::Error::custom)
      }
    }
  };

  stream.into()
}
