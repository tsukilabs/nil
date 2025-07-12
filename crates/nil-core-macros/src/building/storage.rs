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
      use crate::error::Result;
      use crate::infrastructure::storage::{
        Storage,
        StorageCapacity,
        StorageId,
        StorageStatsTable
      };

      impl Storage for #name {
        fn storage_id(&self) -> StorageId {
          Self::STORAGE_ID
        }

        fn capacity(&self, stats: &StorageStatsTable) -> Result<StorageCapacity> {
          Ok(stats.get(self.level)?.capacity)
        }

        fn min_capacity(&self) -> StorageCapacity {
          Self::MIN_CAPACITY
        }

        fn max_capacity(&self) -> StorageCapacity {
          Self::MAX_CAPACITY
        }
      }
    }
  };

  stream.into()
}
