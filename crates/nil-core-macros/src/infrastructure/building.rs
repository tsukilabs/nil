// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[expect(clippy::too_many_lines)]
pub fn impl_building(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    mod __impl_building {
      use super::#name;
      use crate::error::Result;
      use crate::infrastructure::requirements::InfrastructureRequirements;
      use crate::infrastructure::building::{
        Building,
        BuildingId,
        BuildingLevel,
        BuildingStatsTable,
      };
      use crate::resources::{
        Cost,
        Maintenance,
        MaintenanceRatio,
        ResourceRatio,
        Workforce,
      };

      #[bon::bon]
      impl #name {
        pub fn with_level(level: BuildingLevel) -> Self {
          let mut building = Self::default();
          building.set_level(level);
          building
        }

        #[inline]
        pub fn with_min_level() -> Self {
          Self::with_level(Self::MIN_LEVEL)
        }

        #[inline]
        pub fn with_max_level() -> Self {
          Self::with_level(Self::MAX_LEVEL)
        }

        #[inline]
        pub fn with_random_level() -> Self {
          Self::with_random_level_in()
            .min(Self::MIN_LEVEL)
            .max(Self::MAX_LEVEL)
            .call()
        }

        #[builder]
        pub fn with_random_level_in(
          #[builder(default = #name::MIN_LEVEL)] min: BuildingLevel,
          #[builder(default = #name::MAX_LEVEL)] max: BuildingLevel,
        ) -> #name {
          let min = u8::from(min.max(#name::MIN_LEVEL));
          let max = u8::from(max.min(#name::MAX_LEVEL));
          let level = rand::random_range(min..=max);
          #name::with_level(BuildingLevel::new(level))
        }
      }

      impl Building for #name {
        fn id(&self) -> BuildingId {
          Self::ID
        }

        fn is_enabled(&self) -> bool {
          self.enabled
        }

        fn toggle(&mut self, enabled: bool) {
          self.enabled = enabled;
        }

        fn level(&self) -> BuildingLevel {
          self.level
        }

        fn min_level(&self) -> BuildingLevel {
          Self::MIN_LEVEL
        }

        fn max_level(&self) -> BuildingLevel {
          Self::MAX_LEVEL
        }

        fn set_level(&mut self, level: BuildingLevel) {
          self.level = level.clamp(Self::MIN_LEVEL, Self::MAX_LEVEL);
        }

        fn set_min_level(&mut self) {
          self.level = Self::MIN_LEVEL;
        }

        fn set_max_level(&mut self) {
          self.level = Self::MAX_LEVEL;
        }

        fn increase_level(&mut self) {
          self.increase_level_by(1u8)
        }

        fn increase_level_by(&mut self, amount: u8) {
          self.set_level(self.level + amount)
        }

        fn decrease_level(&mut self) {
          self.decrease_level_by(1u8)
        }

        fn decrease_level_by(&mut self, amount: u8) {
          self.set_level(self.level - amount)
        }

        fn min_cost(&self) -> Cost {
          Self::MIN_COST
        }

        fn max_cost(&self) -> Cost {
          Self::MAX_COST
        }

        fn wood_ratio(&self) -> ResourceRatio {
          Self::WOOD_RATIO
        }

        fn stone_ratio(&self) -> ResourceRatio {
          Self::STONE_RATIO
        }

        fn iron_ratio(&self) -> ResourceRatio {
          Self::IRON_RATIO
        }

        fn maintenance(&self, stats: &BuildingStatsTable) -> Result<Maintenance> {
          Ok(stats.get(self.level)?.maintenance)
        }

        fn maintenance_ratio(&self) -> MaintenanceRatio {
          Self::MAINTENANCE_RATIO
        }

        fn min_workforce(&self) -> Workforce {
          Self::MIN_WORKFORCE
        }

        fn max_workforce(&self) -> Workforce {
          Self::MAX_WORKFORCE
        }

        fn infrastructure_requirements(&self) -> &InfrastructureRequirements {
          &Self::INFRASTRUCTURE_REQUIREMENTS
        }
      }
    }
  };

  stream.into()
}
