// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod cost;
mod diff;
mod maintenance;
mod workforce;

use crate::infrastructure::mine::MineProduction;
use crate::village::Stability;
use derive_more::{Deref, Display};
use nil_num::impl_mul_ceil;
use nil_num::ops::MulCeil;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};

pub use cost::{BaseCost, BaseCostGrowth, ResourceRatio};
pub use diff::{FoodDiff, IronDiff, ResourcesDiff, StoneDiff, WoodDiff};
pub use maintenance::{Maintenance, MaintenanceRatio};
pub use workforce::{Workforce, WorkforceGrowth};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Resources {
  pub food: Food,
  pub iron: Iron,
  pub stone: Stone,
  pub wood: Wood,
}

impl Resources {
  /// Quantidade padrão de recursos para um novo jogador.
  pub const PLAYER: Self = Self {
    food: Food::new(1_000),
    iron: Iron::new(1_000),
    stone: Stone::new(1_000),
    wood: Wood::new(1_000),
  };

  /// Retorna `None` se não houver recursos o suficiente.
  pub fn checked_sub(&self, rhs: &Self) -> Option<Self> {
    Some(Self {
      food: self.food.checked_sub(rhs.food)?,
      iron: self.iron.checked_sub(rhs.iron)?,
      stone: self.stone.checked_sub(rhs.stone)?,
      wood: self.wood.checked_sub(rhs.wood)?,
    })
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

macro_rules! decl_resource {
  ($($name:ident),+ $(,)?) => {
    $(
      #[derive(Clone, Copy, Debug, Default, Deref, Display, Deserialize, Serialize)]
      pub struct $name(u32);

      impl $name {
        #[inline]
        pub const fn new(value: u32) -> Self {
          Self(value)
        }

        #[inline]
        pub const fn zero() -> Self {
          Self(0)
        }

        #[inline]
        pub fn checked_sub(self, rhs: Self) -> Option<Self> {
          self.0.checked_sub(rhs.0).map(Self::new)
        }
      }

      impl Add for $name {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
          Self(self.0.saturating_add(rhs.0))
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

      impl SubAssign for $name {
        fn sub_assign(&mut self, rhs: Self) {
          *self = *self - rhs;
        }
      }

      impl From<MineProduction> for $name {
        fn from(value: MineProduction) -> Self {
          Self::new(*value)
        }
      }

      impl From<f64> for $name {
        fn from(value: f64) -> Self {
          Self::new(value as u32)
        }
      }

      impl From<$name> for f64 {
        fn from(value: $name) -> f64 {
          f64::from(value.0)
        }
      }

      impl Mul<f64> for $name {
        type Output = f64;

        fn mul(self, rhs: f64) -> Self::Output {
          f64::from(self.0) * rhs
        }
      }

      impl Mul<$name> for f64 {
        type Output = f64;

        fn mul(self, rhs: $name) -> Self::Output {
          self * f64::from(rhs.0)
        }
      }

      impl Mul<Stability> for $name {
        type Output = $name;

        fn mul(self, rhs: Stability) -> Self::Output {
          Self::from(self.mul_ceil(*rhs))
        }
      }

      impl MulAssign<Stability> for $name {
        fn mul_assign(&mut self, rhs: Stability) {
          *self = *self * rhs;
        }
      }

      impl Div<f64> for $name {
        type Output = f64;

        fn div(self, rhs: f64) -> Self::Output {
          f64::from(self.0) / rhs
        }
      }

      impl Div<$name> for f64 {
        type Output = f64;

        fn div(self, rhs: $name) -> Self::Output {
          self / f64::from(rhs.0)
        }
      }

      impl_mul_ceil!($name);
    )+
  };
}

decl_resource!(Food, Iron, Stone, Wood);
