// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_big_int_u64(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    impl serde::Serialize for #name {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
        S: serde::Serializer,
      {
        serializer.serialize_str(&self.0.to_string())
      }
    }

    impl<'de> serde::Deserialize<'de> for #name {
      fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where
        D: serde::Deserializer<'de>,
      {
        use serde::de::Error as _;
        String::deserialize(deserializer)?
          .parse::<u64>()
          .map(Self)
          .map_err(D::Error::custom)
      }
    }
  };

  stream.into()
}
