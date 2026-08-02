// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_from_world(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    #[automatically_derived]
    impl<T: Into<::nil_core::world::config::WorldId>> From<T> for #name {
      fn from(world: T) -> Self {
        Self { world: world.into() }
      }
    }
  };

  stream.into()
}
