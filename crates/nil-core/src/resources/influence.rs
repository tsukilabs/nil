// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::cost::{Cost, ResourceRatio};
use super::{Food, Iron, Resources, Stone, Wood};
use crate::check_total_resource_ratio;
use derive_more::{Display, From, Into};
use nil_num::triangle::nearest_triangle;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

/// Influence is a special resource which represents the political power of a ruler
/// and is used to determine how many cities they can simultaneously control.
#[derive(
  Clone, Copy, Debug, Display, From, Into, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize,
)]
pub struct Influence(NonZeroU32);

impl Influence {
  pub const MIN: Influence = Influence(NonZeroU32::MIN);
  pub const MAX: Influence = Influence(NonZeroU32::MAX);

  pub const COST: Cost = Cost::new(100_000);

  pub const FOOD_RATIO: ResourceRatio = ResourceRatio::new(0.19);
  pub const IRON_RATIO: ResourceRatio = ResourceRatio::new(0.27);
  pub const STONE_RATIO: ResourceRatio = ResourceRatio::new(0.27);
  pub const WOOD_RATIO: ResourceRatio = ResourceRatio::new(0.27);
}

impl Influence {
  /// # Safety
  ///
  /// Value must not be zero.
  pub const unsafe fn new_unchecked(value: u32) -> Self {
    unsafe { Self(NonZeroU32::new_unchecked(value)) }
  }

  /// Resources required to acquire one unit of influence.
  pub fn resources() -> Resources {
    Resources {
      food: Food::from((Self::COST * Self::FOOD_RATIO).round()),
      iron: Iron::from((Self::COST * Self::IRON_RATIO).round()),
      stone: Stone::from((Self::COST * Self::STONE_RATIO).round()),
      wood: Wood::from((Self::COST * Self::WOOD_RATIO).round()),
    }
  }

  #[inline]
  pub fn city_limit(&self) -> u32 {
    nearest_triangle(self.0.get())
  }
}

impl Default for Influence {
  fn default() -> Self {
    Self::MIN
  }
}

check_total_resource_ratio!(
  Influence::FOOD_RATIO,
  Influence::IRON_RATIO,
  Influence::STONE_RATIO,
  Influence::WOOD_RATIO
);
