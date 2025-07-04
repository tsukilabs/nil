// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_storage(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    mod __impl_storage {
      use super::#name;
      use crate::infrastructure::storage::{
        Storage,
        StorageCapacity,
        StorageCapacityGrowth,
        StorageId
      };

      impl Storage for #name {
        fn storage_id(&self) -> StorageId {
          Self::STORAGE_ID
        }

        fn capacity(&self) -> StorageCapacity {
          Self::CAPACITY
        }

        fn capacity_growth(&self) -> StorageCapacityGrowth {
          Self::CAPACITY_GROWTH
        }
      }
    }
  };

  stream.into()
}
