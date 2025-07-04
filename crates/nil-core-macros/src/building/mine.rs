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
      use crate::error::{Error, Result, WrapOk};
      use crate::infrastructure::building::Building;
      use crate::infrastructure::mine::{
        Mine,
        MineId,
        MineProduction,
        MineProductionGrowth,
        MineStatsTable
      };

      impl Mine for #name {
        fn mine_id(&self) -> MineId {
          Self::MINE_ID
        }

        fn production(&self) -> MineProduction {
          Self::PRODUCTION
        }

        fn production_growth(&self) -> MineProductionGrowth {
          Self::PRODUCTION_GROWTH
        }

        fn current_production(&self, stats: &MineStatsTable) -> Result<MineProduction> {
          Ok(stats.get(self.level)?.production)
        }
      }
    }
  };

  stream.into()
}
