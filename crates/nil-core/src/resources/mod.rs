// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod cost;
mod diff;
mod maintenance;
pub mod prelude;
mod workforce;

use crate::city::Stability;
use crate::infrastructure::mine::MineProduction;
use crate::infrastructure::storage::{OverallStorageCapacity, StorageCapacity};
use derive_more::{Deref, Display, Into};
use nil_num::impl_mul_ceil;
use nil_num::ops::MulCeil;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::num::NonZeroU32;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

pub use cost::{Cost, ResourceRatio};
pub use diff::{FoodDiff, IronDiff, ResourcesDiff, StoneDiff, WoodDiff};
pub use maintenance::{Maintenance, MaintenanceRatio};
pub use workforce::Workforce;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Resources {
  pub food: Food,
  pub iron: Iron,
  pub stone: Stone,
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
  pub const PLAYER: Self = Self {
    food: Food::new(800),
    iron: Iron::new(800),
    stone: Stone::new(800),
    wood: Wood::new(800),
  };

  /// Default amount of resources for a bot.
  pub const BOT: Self = Self {
    food: Food::new(2500),
    iron: Iron::new(2500),
    stone: Stone::new(2500),
    wood: Wood::new(2500),
  };

  /// Default amount of resources for a precursor.
  pub const PRECURSOR: Self = Self {
    food: Food::new(5_000_000),
    iron: Iron::new(5_000_000),
    stone: Stone::new(5_000_000),
    wood: Wood::new(5_000_000),
  };

  #[inline]
  #[must_use]
  pub fn new() -> Self {
    Self::MIN.clone()
  }

  #[inline]
  #[must_use]
  pub fn with_food(&self, food: Food) -> Self {
    Self { food, ..self.clone() }
  }

  #[inline]
  #[must_use]
  pub fn with_iron(&self, iron: Iron) -> Self {
    Self { iron, ..self.clone() }
  }

  #[inline]
  #[must_use]
  pub fn with_stone(&self, stone: Stone) -> Self {
    Self { stone, ..self.clone() }
  }

  #[inline]
  #[must_use]
  pub fn with_wood(&self, wood: Wood) -> Self {
    Self { wood, ..self.clone() }
  }

  pub fn add_within_capacity(&mut self, diff: &ResourcesDiff, capacity: &OverallStorageCapacity) {
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
  pub fn checked_sub(&self, rhs: &Self) -> Option<Self> {
    Some(Self {
      food: self.food.checked_sub(rhs.food)?,
      iron: self.iron.checked_sub(rhs.iron)?,
      stone: self.stone.checked_sub(rhs.stone)?,
      wood: self.wood.checked_sub(rhs.wood)?,
    })
  }
}

impl Default for Resources {
  fn default() -> Self {
    Self::new()
  }
}

impl Add for Resources {
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

impl Add<&Resources> for Resources {
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

impl AddAssign for Resources {
  fn add_assign(&mut self, rhs: Self) {
    *self = Self {
      food: self.food + rhs.food,
      iron: self.iron + rhs.iron,
      stone: self.stone + rhs.stone,
      wood: self.wood + rhs.wood,
    };
  }
}

impl AddAssign<&Resources> for Resources {
  fn add_assign(&mut self, rhs: &Resources) {
    *self = Self {
      food: self.food + rhs.food,
      iron: self.iron + rhs.iron,
      stone: self.stone + rhs.stone,
      wood: self.wood + rhs.wood,
    };
  }
}

impl Sub for Resources {
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

impl Sub<&Resources> for Resources {
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

impl SubAssign for Resources {
  fn sub_assign(&mut self, rhs: Self) {
    *self = Self {
      food: self.food - rhs.food,
      iron: self.iron - rhs.iron,
      stone: self.stone - rhs.stone,
      wood: self.wood - rhs.wood,
    };
  }
}

impl SubAssign<&Resources> for Resources {
  fn sub_assign(&mut self, rhs: &Resources) {
    *self = Self {
      food: self.food - rhs.food,
      iron: self.iron - rhs.iron,
      stone: self.stone - rhs.stone,
      wood: self.wood - rhs.wood,
    };
  }
}

impl Mul<u32> for &Resources {
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

impl Mul<NonZeroU32> for &Resources {
  type Output = Resources;

  fn mul(self, rhs: NonZeroU32) -> Self::Output {
    self * rhs.get()
  }
}

macro_rules! decl_resource {
  ($($name:ident => $diff:ident),+ $(,)?) => {
    $(
      #[derive(
        Clone,
        Copy,
        Debug,
        Default,
        Deref,
        Display,
        Into,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Deserialize,
        Serialize,
      )]
      #[into(u32, f64)]
      pub struct $name(u32);

      impl $name {
        pub const MIN: Self = Self::new(0);
        pub const MAX: Self = Self::new(u32::MAX);

        #[inline]
        pub const fn new(value: u32) -> Self {
          Self(value)
        }

        #[inline]
        pub fn checked_sub(self, rhs: Self) -> Option<Self> {
          self.0.checked_sub(rhs.0).map(Self::new)
        }

        pub fn add_within_capacity(&mut self, diff: $diff, capacity: StorageCapacity) {
          if diff < 0i32 {
            *self += diff;
          } else if self.0 < *capacity {
            let capacity = $name::from(capacity);
            *self = (*self + diff).min(capacity);
          }
        }
      }

      impl From<u32> for $name {
        fn from(value: u32) -> Self {
          Self(value)
        }
      }

      impl From<f64> for $name {
        fn from(value: f64) -> Self {
          debug_assert!(value.is_finite());
          Self(value as u32)
        }
      }

      impl From<MineProduction> for $name {
        fn from(value: MineProduction) -> Self {
          Self(*value)
        }
      }

      impl From<StorageCapacity> for $name {
        fn from(value: StorageCapacity) -> Self {
          Self(*value)
        }
      }

      impl PartialEq<u32> for $name {
        fn eq(&self, other: &u32) -> bool {
          self.0.eq(other)
        }
      }

      impl PartialOrd<u32> for $name {
        fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
          self.0.partial_cmp(other)
        }
      }

      impl Add for $name {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
          Self(self.0.saturating_add(rhs.0))
        }
      }

      impl Add<u32> for $name {
        type Output = Self;

        fn add(self, rhs: u32) -> Self {
          Self(self.0.saturating_add(rhs))
        }
      }

      impl AddAssign for $name {
        fn add_assign(&mut self, rhs: Self) {
          *self = *self + rhs;
        }
      }

      impl Sub for $name {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self {
          Self(self.0.saturating_sub(rhs.0))
        }
      }

      impl Sub<u32> for $name {
        type Output = Self;

        fn sub(self, rhs: u32) -> Self {
          Self(self.0.saturating_sub(rhs))
        }
      }

      impl SubAssign for $name {
        fn sub_assign(&mut self, rhs: Self) {
          *self = *self - rhs;
        }
      }

      impl Mul<u32> for $name {
        type Output = Self;

        fn mul(self, rhs: u32) -> Self::Output {
          Self(self.0.saturating_mul(rhs))
        }
      }

      impl Mul<NonZeroU32> for $name {
        type Output = Self;

        fn mul(self, rhs: NonZeroU32) -> Self::Output {
          self * rhs.get()
        }
      }

      impl Mul<f64> for $name {
        type Output = f64;

        fn mul(self, rhs: f64) -> Self::Output {
          f64::from(self.0) * rhs
        }
      }

      impl Mul<Stability> for $name {
        type Output = $name;

        fn mul(self, rhs: Stability) -> Self::Output {
          Self::from(self.mul_ceil(*rhs))
        }
      }

      impl MulAssign<u32> for $name {
        fn mul_assign(&mut self, rhs: u32) {
          *self = *self * rhs;
        }
      }

      impl MulAssign<Stability> for $name {
        fn mul_assign(&mut self, rhs: Stability) {
          *self = *self * rhs;
        }
      }

      impl_mul_ceil!($name);
    )+
  };
}

decl_resource!(Food => FoodDiff, Iron => IronDiff, Stone => StoneDiff, Wood => WoodDiff);
