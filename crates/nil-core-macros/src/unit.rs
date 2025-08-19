// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_unit(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    mod __impl_unit {
      use super::#name;
      use crate::infrastructure::requirements::InfrastructureRequirements;
      use crate::military::unit::{Unit, UnitBox, UnitChunk, UnitId, UnitKind};
      use crate::military::unit::stats::prelude::*;
      use crate::ranking::Score;

      impl #name {
        pub fn new_boxed() -> UnitBox {
          UnitBox::new(Box::new(Self))
        }
      }

      impl Unit for #name {
        fn id(&self) -> UnitId {
          Self::ID
        }

        fn kind(&self) -> UnitKind {
          Self::KIND
        }

        fn score(&self) -> Score {
          Self::SCORE
        }

        fn stats(&self) -> &UnitStats {
          &Self::STATS
        }

        fn attack(&self) -> Power {
          Self::STATS.attack()
        }

        fn infantry_defense(&self) -> Power {
          Self::STATS.infantry_defense()
        }

        fn cavalry_defense(&self) -> Power {
          Self::STATS.cavalry_defense()
        }

        fn ranged_defense(&self) -> Power {
          Self::STATS.ranged_defense()
        }

        fn ranged_debuff(&self) -> RangedDebuff {
          Self::STATS.ranged_debuff()
        }

        fn speed(&self) -> Speed {
          Self::STATS.speed()
        }

        fn haul(&self) -> Haul {
          Self::STATS.haul()
        }

        fn chunk(&self) -> &UnitChunk {
          &Self::CHUNK
        }

        fn infrastructure_requirements(&self) -> &InfrastructureRequirements {
          &Self::INFRASTRUCTURE_REQUIREMENTS
        }
      }

      impl From<#name> for UnitId {
        fn from(_: #name) -> UnitId {
          #name::ID
        }
      }
    }
  };

  stream.into()
}
