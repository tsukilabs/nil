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
  ($($original:ident => $diff:ident),+ $(,)?) => {
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
      pub struct $diff(i32);

      impl $diff {
        #[inline]
        pub const fn new(value: i32) -> Self {
          Self(value)
        }

        #[inline]
        pub const fn zero() -> Self {
          Self(0)
        }
      }

      impl PartialEq<i32> for $diff {
        fn eq(&self, other: &i32) -> bool {
          self.0.eq(other)
        }
      }

      impl PartialOrd<i32> for $diff {
        fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
          self.0.partial_cmp(other)
        }
      }

      impl Add for $diff {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
          Self(self.0.saturating_add(rhs.0))
        }
      }

      impl Add<$original> for $diff {
        type Output = Self;

        fn add(self, rhs: $original) -> Self {
          Self(self.0.saturating_add_unsigned(rhs.0))
        }
      }

      impl Add<$diff> for $original {
        type Output = Self;

        fn add(self, rhs: $diff) -> Self {
          Self(self.0.saturating_add_signed(rhs.0))
        }
      }

      impl AddAssign for $diff {
        fn add_assign(&mut self, rhs: Self) {
          *self = *self + rhs;
        }
      }

      impl AddAssign<$original> for $diff {
        fn add_assign(&mut self, rhs: $original) {
          *self = *self + rhs;
        }
      }

      impl AddAssign<$diff> for $original {
        fn add_assign(&mut self, rhs: $diff) {
          *self = *self + rhs;
        }
      }

      impl Sub for $diff {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self {
          Self(self.0.saturating_sub(rhs.0))
        }
      }

      impl Sub<$original> for $diff {
        type Output = Self;

        fn sub(self, rhs: $original) -> Self {
          Self(self.0.saturating_sub_unsigned(rhs.0))
        }
      }

      impl Sub<$diff> for $original {
        type Output = Self;

        fn sub(self, rhs: $diff) -> Self {
          Self(self.0.saturating_sub_signed(rhs.0))
        }
      }

      impl SubAssign for $diff {
        fn sub_assign(&mut self, rhs: Self) {
          *self = *self - rhs;
        }
      }

      impl SubAssign<$original> for $diff {
        fn sub_assign(&mut self, rhs: $original) {
          *self = *self - rhs;
        }
      }

      impl SubAssign<$diff> for $original {
        fn sub_assign(&mut self, rhs: $diff) {
          *self = *self - rhs;
        }
      }
    )+
  }
}

decl_resource_diff!(Food => FoodDiff, Iron => IronDiff, Stone => StoneDiff, Wood => WoodDiff);
