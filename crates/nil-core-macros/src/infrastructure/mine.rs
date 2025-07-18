// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_mine(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    mod __impl_mine {
      use super::#name;
      use crate::error::Result;
      use crate::infrastructure::building::MineId;
      use crate::infrastructure::mine::{Mine, MineProduction, MineStatsTable};

      impl Mine for #name {
        fn mine_id(&self) -> MineId {
          Self::MINE_ID
        }

        fn production(&self, stats: &MineStatsTable) -> Result<MineProduction> {
          Ok(stats.get(self.level)?.production)
        }

        fn min_production(&self) -> MineProduction {
          Self::MIN_PRODUCTION
        }

        fn max_production(&self) -> MineProduction {
          Self::MAX_PRODUCTION
        }
      }
    }
  };

  stream.into()
}
