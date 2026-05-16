// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod cost;
pub mod diff;
pub mod influence;
pub mod maintenance;
pub mod prelude;
pub mod workforce;

use crate::city::stability::Stability;
use crate::infrastructure::mine::MineProduction;
use crate::infrastructure::storage::{OverallStorageCapacity, StorageCapacity};
use bon::Builder;
use derive_more::Display;
use diff::{FoodDiff, IronDiff, ResourcesDiff, StoneDiff, WoodDiff};
use nil_num::impl_mul_ceil;
use nil_num::mul_ceil::MulCeil;
use nil_util::{ConstDeref, F64Math};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::num::NonZeroU32;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Builder, Debug, Deserialize, Serialize)]
#[derive_const(Clone, PartialEq, Eq)]
#[serde(default, rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct Resources {
  #[builder(default)]
  pub food: Food,

  #[builder(default)]
  pub iron: Iron,

  #[builder(default)]
  pub stone: Stone,

  #[builder(default)]
  pub wood: Wood,
}

impl Resources {
  /// Minimum possible amount of resources.
  pub const MIN: Self = Self {
    food: Food::MIN,
    iron: Iron::MIN,
    stone: Stone::MIN,
    wood: Wood::MIN,
  };

  /// Maximum possible amount of resources.
  pub const MAX: Self = Self {
    food: Food::MAX,
    iron: Iron::MAX,
    stone: Stone::MAX,
    wood: Wood::MAX,
  };

  /// Default amount of resources for a player.
  pub const PLAYER: Self = Self::splat(800);

  /// Default amount of resources for a bot.
  pub const BOT: Self = Self::splat(2500);

  /// Default amount of resources for a precursor.
  pub const PRECURSOR: Self = Self::splat(5_000_000);

  #[inline]
  #[must_use]
  pub const fn new() -> Self {
    Self::MIN.clone()
  }

  #[must_use]
  pub const fn splat(value: u32) -> Self {
    Self {
      food: Food::new(value),
      iron: Iron::new(value),
      stone: Stone::new(value),
      wood: Wood::new(value),
    }
  }

  #[inline]
  #[must_use]
  pub const fn with_food(&self, food: Food) -> Self {
    Self { food, ..self.clone() }
  }

  #[inline]
  #[must_use]
  pub const fn with_iron(&self, iron: Iron) -> Self {
    Self { iron, ..self.clone() }
  }

  #[inline]
  #[must_use]
  pub const fn with_stone(&self, stone: Stone) -> Self {
    Self { stone, ..self.clone() }
  }

  #[inline]
  #[must_use]
  pub const fn with_wood(&self, wood: Wood) -> Self {
    Self { wood, ..self.clone() }
  }

  #[must_use]
  pub const fn silo(&self) -> Self {
    Self {
      food: self.food,
      iron: Iron::MIN,
      stone: Stone::MIN,
      wood: Wood::MIN,
    }
  }

  #[must_use]
  pub const fn warehouse(&self) -> Self {
    Self {
      food: Food::MIN,
      iron: self.iron,
      stone: self.stone,
      wood: self.wood,
    }
  }

  pub const fn add_within_capacity(
    &mut self,
    diff: &ResourcesDiff,
    capacity: &OverallStorageCapacity,
  ) {
    macro_rules! add {
      ($($resource:ident => $storage:ident),+ $(,)?) => {
        $(
          let resource = diff.$resource;
          let storage = capacity.$storage;
          self.$resource.add_within_capacity(resource, storage);
        )+
      };
    }

    add!(food => silo, iron => warehouse, stone => warehouse, wood => warehouse);
  }

  /// Checked resource subtraction.
  /// Returns `None` if there are not enough resources available.
  pub const fn checked_sub(&self, rhs: &Self) -> Option<Self> {
    Some(Self {
      food: self.food.checked_sub(rhs.food)?,
      iron: self.iron.checked_sub(rhs.iron)?,
      stone: self.stone.checked_sub(rhs.stone)?,
      wood: self.wood.checked_sub(rhs.wood)?,
    })
  }

  pub const fn sum(&self) -> u32 {
    0u32
      .saturating_add(self.food.0)
      .saturating_add(self.iron.0)
      .saturating_add(self.stone.0)
      .saturating_add(self.wood.0)
  }

  #[inline]
  pub const fn sum_silo(&self) -> u32 {
    self.silo().sum()
  }

  #[inline]
  pub const fn sum_warehouse(&self) -> u32 {
    self.warehouse().sum()
  }

  #[inline]
  pub const fn is_empty(&self) -> bool {
    self.sum() == 0
  }
}

impl const Default for Resources {
  fn default() -> Self {
    Self::new()
  }
}

impl const Add for Resources {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    Self {
      food: self.food + rhs.food,
      iron: self.iron + rhs.iron,
      stone: self.stone + rhs.stone,
      wood: self.wood + rhs.wood,
    }
  }
}

impl const Add<&Resources> for Resources {
  type Output = Self;

  fn add(self, rhs: &Resources) -> Self {
    Self {
      food: self.food + rhs.food,
      iron: self.iron + rhs.iron,
      stone: self.stone + rhs.stone,
      wood: self.wood + rhs.wood,
    }
  }
}

impl const AddAssign for Resources {
  fn add_assign(&mut self, rhs: Self) {
    *self = Self {
      food: self.food + rhs.food,
      iron: self.iron + rhs.iron,
      stone: self.stone + rhs.stone,
      wood: self.wood + rhs.wood,
    };
  }
}

impl const AddAssign<&Resources> for Resources {
  fn add_assign(&mut self, rhs: &Resources) {
    *self = Self {
      food: self.food + rhs.food,
      iron: self.iron + rhs.iron,
      stone: self.stone + rhs.stone,
      wood: self.wood + rhs.wood,
    };
  }
}

impl const Sub for Resources {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self {
    Self {
      food: self.food - rhs.food,
      iron: self.iron - rhs.iron,
      stone: self.stone - rhs.stone,
      wood: self.wood - rhs.wood,
    }
  }
}

impl const Sub<&Resources> for Resources {
  type Output = Self;

  fn sub(self, rhs: &Resources) -> Self {
    Self {
      food: self.food - rhs.food,
      iron: self.iron - rhs.iron,
      stone: self.stone - rhs.stone,
      wood: self.wood - rhs.wood,
    }
  }
}

impl const SubAssign for Resources {
  fn sub_assign(&mut self, rhs: Self) {
    *self = Self {
      food: self.food - rhs.food,
      iron: self.iron - rhs.iron,
      stone: self.stone - rhs.stone,
      wood: self.wood - rhs.wood,
    };
  }
}

impl const SubAssign<&Resources> for Resources {
  fn sub_assign(&mut self, rhs: &Resources) {
    *self = Self {
      food: self.food - rhs.food,
      iron: self.iron - rhs.iron,
      stone: self.stone - rhs.stone,
      wood: self.wood - rhs.wood,
    };
  }
}

impl const Mul<u32> for &Resources {
  type Output = Resources;

  fn mul(self, rhs: u32) -> Self::Output {
    Resources {
      food: self.food * rhs,
      iron: self.iron * rhs,
      stone: self.stone * rhs,
      wood: self.wood * rhs,
    }
  }
}

impl const Mul<NonZeroU32> for &Resources {
  type Output = Resources;

  fn mul(self, rhs: NonZeroU32) -> Self::Output {
    self * rhs.get()
  }
}

macro_rules! decl_resource {
  ($($resource:ident),+ $(,)?) => {
    paste::paste! {
      $(
        #[derive(Copy, Debug, Display, Deserialize, Serialize, ConstDeref, F64Math)]
        #[derive_const(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
        #[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
        pub struct $resource(u32);

        impl $resource {
          pub const MIN: Self = Self::new(0);
          pub const MAX: Self = Self::new(u32::MAX);

          #[inline]
          pub const fn new(value: u32) -> Self {
            Self(value)
          }

          #[inline]
          pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
            self.0.checked_sub(rhs.0).map(Self::new)
          }

          pub const fn add_within_capacity(
            &mut self,
            diff: [<$resource Diff>],
            capacity: StorageCapacity
          ) {
            if diff < 0i32 {
              *self += diff;
            } else if self.0 < *capacity {
              let capacity = $resource::from(capacity);
              *self = (*self + diff).min(capacity);
            }
          }
        }

        impl const From<u32> for $resource {
          fn from(value: u32) -> Self {
            Self::new(value)
          }
        }

        impl const From<$resource> for u32 {
          fn from(value: $resource) -> Self {
            value.0
          }
        }

        impl const From<f64> for $resource {
          fn from(value: f64) -> Self {
            debug_assert!(value.is_finite());
            debug_assert!(value >= 0.0);
            Self(value.trunc() as u32)
          }
        }

        impl const From<$resource> for f64 {
          fn from(value: $resource) -> Self {
            f64::from(value.0)
          }
        }

        impl const From<MineProduction> for $resource {
          fn from(value: MineProduction) -> Self {
            Self(*value)
          }
        }

        impl const From<StorageCapacity> for $resource {
          fn from(value: StorageCapacity) -> Self {
            Self(*value)
          }
        }

        impl const PartialEq<u32> for $resource {
          fn eq(&self, other: &u32) -> bool {
            self.0.eq(other)
          }
        }

        impl const PartialOrd<u32> for $resource {
          fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
            self.0.partial_cmp(other)
          }
        }

        impl const Add for $resource {
          type Output = Self;

          fn add(self, rhs: Self) -> Self {
            Self(self.0.saturating_add(rhs.0))
          }
        }

        impl const Add<u32> for $resource {
          type Output = Self;

          fn add(self, rhs: u32) -> Self {
            Self(self.0.saturating_add(rhs))
          }
        }

        impl const AddAssign for $resource {
          fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
          }
        }

        impl const Sub for $resource {
          type Output = Self;

          fn sub(self, rhs: Self) -> Self {
            Self(self.0.saturating_sub(rhs.0))
          }
        }

        impl const Sub<u32> for $resource {
          type Output = Self;

          fn sub(self, rhs: u32) -> Self {
            Self(self.0.saturating_sub(rhs))
          }
        }

        impl const SubAssign for $resource {
          fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs;
          }
        }

        impl const Mul<u32> for $resource {
          type Output = Self;

          fn mul(self, rhs: u32) -> Self::Output {
            Self(self.0.saturating_mul(rhs))
          }
        }

        impl const Mul<NonZeroU32> for $resource {
          type Output = Self;

          fn mul(self, rhs: NonZeroU32) -> Self::Output {
            self * rhs.get()
          }
        }

        impl const Mul<Stability> for $resource {
          type Output = $resource;

          fn mul(self, rhs: Stability) -> Self::Output {
            Self::from(self.mul_ceil(*rhs))
          }
        }

        impl const MulAssign<u32> for $resource {
          fn mul_assign(&mut self, rhs: u32) {
            *self = *self * rhs;
          }
        }

        impl const MulAssign<Stability> for $resource {
          fn mul_assign(&mut self, rhs: Stability) {
            *self = *self * rhs;
          }
        }

        impl_mul_ceil!($resource);
      )+
    }
  };
}

decl_resource!(Food, Iron, Stone, Wood);
