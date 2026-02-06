// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::{Food, Iron, Resources, Stone, Wood};
use derive_more::{Deref, Display, Into};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ResourcesDiff {
  pub food: FoodDiff,
  pub iron: IronDiff,
  pub stone: StoneDiff,
  pub wood: WoodDiff,
}

impl Add for ResourcesDiff {
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

impl Add<Resources> for ResourcesDiff {
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

impl Add<ResourcesDiff> for Resources {
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

impl AddAssign for ResourcesDiff {
  fn add_assign(&mut self, rhs: Self) {
    *self = Self {
      food: self.food + rhs.food,
      iron: self.iron + rhs.iron,
      stone: self.stone + rhs.stone,
      wood: self.wood + rhs.wood,
    };
  }
}

impl AddAssign<Resources> for ResourcesDiff {
  fn add_assign(&mut self, rhs: Resources) {
    *self = Self {
      food: self.food + rhs.food,
      iron: self.iron + rhs.iron,
      stone: self.stone + rhs.stone,
      wood: self.wood + rhs.wood,
    };
  }
}

impl AddAssign<ResourcesDiff> for Resources {
  fn add_assign(&mut self, rhs: ResourcesDiff) {
    *self = Self {
      food: self.food + rhs.food,
      iron: self.iron + rhs.iron,
      stone: self.stone + rhs.stone,
      wood: self.wood + rhs.wood,
    };
  }
}

impl Sub for ResourcesDiff {
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

impl Sub<ResourcesDiff> for Resources {
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

impl SubAssign for ResourcesDiff {
  fn sub_assign(&mut self, rhs: Self) {
    *self = Self {
      food: self.food - rhs.food,
      iron: self.iron - rhs.iron,
      stone: self.stone - rhs.stone,
      wood: self.wood - rhs.wood,
    };
  }
}

impl SubAssign<Resources> for ResourcesDiff {
  fn sub_assign(&mut self, rhs: Resources) {
    *self = Self {
      food: self.food - rhs.food,
      iron: self.iron - rhs.iron,
      stone: self.stone - rhs.stone,
      wood: self.wood - rhs.wood,
    };
  }
}

impl SubAssign<ResourcesDiff> for Resources {
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
        #[into(i32, f64)]
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

        impl PartialEq<i32> for [<$resource Diff>] {
          fn eq(&self, other: &i32) -> bool {
            self.0.eq(other)
          }
        }

        impl PartialOrd<i32> for [<$resource Diff>] {
          fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
            self.0.partial_cmp(other)
          }
        }

        impl Add for [<$resource Diff>] {
          type Output = Self;

          fn add(self, rhs: Self) -> Self {
            Self(self.0.saturating_add(rhs.0))
          }
        }

        impl Add<$resource> for [<$resource Diff>] {
          type Output = Self;

          fn add(self, rhs: $resource) -> Self {
            Self(self.0.saturating_add_unsigned(rhs.0))
          }
        }

        impl Add<[<$resource Diff>]> for $resource {
          type Output = Self;

          fn add(self, rhs: [<$resource Diff>]) -> Self {
            Self(self.0.saturating_add_signed(rhs.0))
          }
        }

        impl Add<i32> for [<$resource Diff>] {
          type Output = Self;

          fn add(self, rhs: i32) -> Self {
            Self(self.0.saturating_add(rhs))
          }
        }

        impl Add<u32> for [<$resource Diff>] {
          type Output = Self;

          fn add(self, rhs: u32) -> Self {
            Self(self.0.saturating_add_unsigned(rhs))
          }
        }

        impl AddAssign for [<$resource Diff>] {
          fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
          }
        }

        impl AddAssign<$resource> for [<$resource Diff>] {
          fn add_assign(&mut self, rhs: $resource) {
            *self = *self + rhs;
          }
        }

        impl AddAssign<[<$resource Diff>]> for $resource {
          fn add_assign(&mut self, rhs: [<$resource Diff>]) {
            *self = *self + rhs;
          }
        }

        impl AddAssign<i32> for [<$resource Diff>] {
          fn add_assign(&mut self, rhs: i32) {
            *self = *self + rhs;
          }
        }

        impl AddAssign<u32> for [<$resource Diff>] {
          fn add_assign(&mut self, rhs: u32) {
            *self = *self + rhs;
          }
        }

        impl Sub for [<$resource Diff>] {
          type Output = Self;

          fn sub(self, rhs: Self) -> Self {
            Self(self.0.saturating_sub(rhs.0))
          }
        }

        impl Sub<$resource> for [<$resource Diff>] {
          type Output = Self;

          fn sub(self, rhs: $resource) -> Self {
            Self(self.0.saturating_sub_unsigned(rhs.0))
          }
        }

        impl Sub<[<$resource Diff>]> for $resource {
          type Output = Self;

          fn sub(self, rhs: [<$resource Diff>]) -> Self {
            Self(self.0.saturating_sub_signed(rhs.0))
          }
        }

        impl Sub<i32> for [<$resource Diff>] {
          type Output = Self;

          fn sub(self, rhs: i32) -> Self {
            Self(self.0.saturating_sub(rhs))
          }
        }

        impl Sub<u32> for [<$resource Diff>] {
          type Output = Self;

          fn sub(self, rhs: u32) -> Self {
            Self(self.0.saturating_sub_unsigned(rhs))
          }
        }

        impl SubAssign for [<$resource Diff>] {
          fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs;
          }
        }

        impl SubAssign<$resource> for [<$resource Diff>] {
          fn sub_assign(&mut self, rhs: $resource) {
            *self = *self - rhs;
          }
        }

        impl SubAssign<[<$resource Diff>]> for $resource {
          fn sub_assign(&mut self, rhs: [<$resource Diff>]) {
            *self = *self - rhs;
          }
        }

        impl SubAssign<i32> for [<$resource Diff>] {
          fn sub_assign(&mut self, rhs: i32) {
            *self = *self - rhs;
          }
        }

        impl SubAssign<u32> for [<$resource Diff>] {
          fn sub_assign(&mut self, rhs: u32) {
            *self = *self - rhs;
          }
        }
      )+
    }
  }
}

decl_resource_diff!(Food, Iron, Stone, Wood);
