// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::{Food, Iron, Resources, Stone, Wood};
use derive_more::Display;
use nil_num::F64Math;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Deref, Sub, SubAssign};

#[derive(Debug, Deserialize, Serialize)]
#[derive_const(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[serde(default, rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct ResourcesDiff {
  pub food: FoodDiff,
  pub iron: IronDiff,
  pub stone: StoneDiff,
  pub wood: WoodDiff,
}

impl const Add for ResourcesDiff {
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

impl const Add<Resources> for ResourcesDiff {
  type Output = Self;

  fn add(self, rhs: Resources) -> Self {
    Self {
      food: self.food + rhs.food,
      iron: self.iron + rhs.iron,
      stone: self.stone + rhs.stone,
      wood: self.wood + rhs.wood,
    }
  }
}

impl const Add<ResourcesDiff> for Resources {
  type Output = Self;

  fn add(self, rhs: ResourcesDiff) -> Self {
    Self {
      food: self.food + rhs.food,
      iron: self.iron + rhs.iron,
      stone: self.stone + rhs.stone,
      wood: self.wood + rhs.wood,
    }
  }
}

impl const AddAssign for ResourcesDiff {
  fn add_assign(&mut self, rhs: Self) {
    *self = Self {
      food: self.food + rhs.food,
      iron: self.iron + rhs.iron,
      stone: self.stone + rhs.stone,
      wood: self.wood + rhs.wood,
    };
  }
}

impl const AddAssign<Resources> for ResourcesDiff {
  fn add_assign(&mut self, rhs: Resources) {
    *self = Self {
      food: self.food + rhs.food,
      iron: self.iron + rhs.iron,
      stone: self.stone + rhs.stone,
      wood: self.wood + rhs.wood,
    };
  }
}

impl const AddAssign<ResourcesDiff> for Resources {
  fn add_assign(&mut self, rhs: ResourcesDiff) {
    *self = Self {
      food: self.food + rhs.food,
      iron: self.iron + rhs.iron,
      stone: self.stone + rhs.stone,
      wood: self.wood + rhs.wood,
    };
  }
}

impl const Sub for ResourcesDiff {
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

impl const Sub<ResourcesDiff> for Resources {
  type Output = Self;

  fn sub(self, rhs: ResourcesDiff) -> Self {
    Self {
      food: self.food - rhs.food,
      iron: self.iron - rhs.iron,
      stone: self.stone - rhs.stone,
      wood: self.wood - rhs.wood,
    }
  }
}

impl const SubAssign for ResourcesDiff {
  fn sub_assign(&mut self, rhs: Self) {
    *self = Self {
      food: self.food - rhs.food,
      iron: self.iron - rhs.iron,
      stone: self.stone - rhs.stone,
      wood: self.wood - rhs.wood,
    };
  }
}

impl const SubAssign<Resources> for ResourcesDiff {
  fn sub_assign(&mut self, rhs: Resources) {
    *self = Self {
      food: self.food - rhs.food,
      iron: self.iron - rhs.iron,
      stone: self.stone - rhs.stone,
      wood: self.wood - rhs.wood,
    };
  }
}

impl const SubAssign<ResourcesDiff> for Resources {
  fn sub_assign(&mut self, rhs: ResourcesDiff) {
    *self = Self {
      food: self.food - rhs.food,
      iron: self.iron - rhs.iron,
      stone: self.stone - rhs.stone,
      wood: self.wood - rhs.wood,
    };
  }
}

macro_rules! decl_resource_diff {
  ($($resource:ident),+ $(,)?) => {
    paste::paste! {
      $(
        #[derive(Copy, Debug, Display, Deserialize, Serialize, F64Math)]
        #[derive_const(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
        #[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
        pub struct [<$resource Diff>](i32);

        impl [<$resource Diff>] {
          #[inline]
          pub const fn new(value: i32) -> Self {
            Self(value)
          }

          #[inline]
          pub const fn zero() -> Self {
            Self(0)
          }
        }

        impl const Deref for [<$resource Diff>] {
          type Target = i32;

          fn deref(&self) -> &Self::Target {
            &self.0
          }
        }

        impl const From<[<$resource Diff>]> for f64 {
          fn from(value: [<$resource Diff>]) -> Self {
            f64::from(value.0)
          }
        }

        impl const PartialEq<i32> for [<$resource Diff>] {
          fn eq(&self, other: &i32) -> bool {
            self.0.eq(other)
          }
        }

        impl const PartialOrd<i32> for [<$resource Diff>] {
          fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
            self.0.partial_cmp(other)
          }
        }

        impl const Add for [<$resource Diff>] {
          type Output = Self;

          fn add(self, rhs: Self) -> Self {
            Self(self.0.saturating_add(rhs.0))
          }
        }

        impl const Add<$resource> for [<$resource Diff>] {
          type Output = Self;

          fn add(self, rhs: $resource) -> Self {
            Self(self.0.saturating_add_unsigned(rhs.0))
          }
        }

        impl const Add<[<$resource Diff>]> for $resource {
          type Output = Self;

          fn add(self, rhs: [<$resource Diff>]) -> Self {
            Self(self.0.saturating_add_signed(rhs.0))
          }
        }

        impl const Add<i32> for [<$resource Diff>] {
          type Output = Self;

          fn add(self, rhs: i32) -> Self {
            Self(self.0.saturating_add(rhs))
          }
        }

        impl const Add<u32> for [<$resource Diff>] {
          type Output = Self;

          fn add(self, rhs: u32) -> Self {
            Self(self.0.saturating_add_unsigned(rhs))
          }
        }

        impl const AddAssign for [<$resource Diff>] {
          fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
          }
        }

        impl const AddAssign<$resource> for [<$resource Diff>] {
          fn add_assign(&mut self, rhs: $resource) {
            *self = *self + rhs;
          }
        }

        impl const AddAssign<[<$resource Diff>]> for $resource {
          fn add_assign(&mut self, rhs: [<$resource Diff>]) {
            *self = *self + rhs;
          }
        }

        impl const AddAssign<i32> for [<$resource Diff>] {
          fn add_assign(&mut self, rhs: i32) {
            *self = *self + rhs;
          }
        }

        impl const AddAssign<u32> for [<$resource Diff>] {
          fn add_assign(&mut self, rhs: u32) {
            *self = *self + rhs;
          }
        }

        impl const Sub for [<$resource Diff>] {
          type Output = Self;

          fn sub(self, rhs: Self) -> Self {
            Self(self.0.saturating_sub(rhs.0))
          }
        }

        impl const Sub<$resource> for [<$resource Diff>] {
          type Output = Self;

          fn sub(self, rhs: $resource) -> Self {
            Self(self.0.saturating_sub_unsigned(rhs.0))
          }
        }

        impl const Sub<[<$resource Diff>]> for $resource {
          type Output = Self;

          fn sub(self, rhs: [<$resource Diff>]) -> Self {
            Self(self.0.saturating_sub_signed(rhs.0))
          }
        }

        impl const Sub<i32> for [<$resource Diff>] {
          type Output = Self;

          fn sub(self, rhs: i32) -> Self {
            Self(self.0.saturating_sub(rhs))
          }
        }

        impl const Sub<u32> for [<$resource Diff>] {
          type Output = Self;

          fn sub(self, rhs: u32) -> Self {
            Self(self.0.saturating_sub_unsigned(rhs))
          }
        }

        impl const SubAssign for [<$resource Diff>] {
          fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs;
          }
        }

        impl const SubAssign<$resource> for [<$resource Diff>] {
          fn sub_assign(&mut self, rhs: $resource) {
            *self = *self - rhs;
          }
        }

        impl const SubAssign<[<$resource Diff>]> for $resource {
          fn sub_assign(&mut self, rhs: [<$resource Diff>]) {
            *self = *self - rhs;
          }
        }

        impl const SubAssign<i32> for [<$resource Diff>] {
          fn sub_assign(&mut self, rhs: i32) {
            *self = *self - rhs;
          }
        }

        impl const SubAssign<u32> for [<$resource Diff>] {
          fn sub_assign(&mut self, rhs: u32) {
            *self = *self - rhs;
          }
        }
      )+
    }
  }
}

decl_resource_diff!(Food, Iron, Stone, Wood);
