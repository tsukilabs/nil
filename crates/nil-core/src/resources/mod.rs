// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod cost;
pub mod diff;
pub mod gold;
pub mod influence;
pub mod maintenance;
pub mod prelude;
pub mod workforce;

use crate::city::stability::Stability;
use crate::error::{Error, Result};
use crate::infrastructure::mine::MineProduction;
use crate::infrastructure::storage::{OverallStorageCapacity, StorageCapacity};
use crate::market::fee::MarketFee;
use crate::resources::gold::Gold;
use bon::Builder;
use derive_more::Display;
use diff::{FoodDiff, IronDiff, ResourcesDiff, StoneDiff, WoodDiff};
use nil_num::impl_mul_ceil;
use nil_num::mul_ceil::MulCeil;
use nil_util::{ConstDeref, F64Math};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::iter::Sum;
use std::num::NonZeroU32;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use strum::{EnumIs, EnumIter};
use subenum::subenum;

/// Basic resources, such as [food].
///
/// [food]: crate::resources::Food
#[derive(Builder, Copy, Debug, Deserialize, Serialize)]
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
    Self::MIN
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

  pub fn with_resource<T>(resource: &T) -> Self
  where
    T: Resource,
  {
    match resource.id() {
      ResourceId::Food => Self::with_food(Food::new(resource.as_u32())),
      ResourceId::Iron => Self::with_iron(Iron::new(resource.as_u32())),
      ResourceId::Stone => Self::with_stone(Stone::new(resource.as_u32())),
      ResourceId::Wood => Self::with_wood(Wood::new(resource.as_u32())),
    }
  }

  #[inline]
  #[must_use]
  pub const fn with_food(food: Food) -> Self {
    Self { food, ..Self::default() }
  }

  #[inline]
  #[must_use]
  pub const fn with_iron(iron: Iron) -> Self {
    Self { iron, ..Self::default() }
  }

  #[inline]
  #[must_use]
  pub const fn with_stone(stone: Stone) -> Self {
    Self { stone, ..Self::default() }
  }

  #[inline]
  #[must_use]
  pub const fn with_wood(wood: Wood) -> Self {
    Self { wood, ..Self::default() }
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

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.sum() == 0
  }

  pub fn replace(&mut self, resources: impl Into<Resources>) {
    *self = resources.into();
  }

  pub const fn get(&self, id: ResourceId) -> &dyn Resource {
    match id {
      ResourceId::Food => self.food.as_dyn(),
      ResourceId::Iron => self.iron.as_dyn(),
      ResourceId::Stone => self.stone.as_dyn(),
      ResourceId::Wood => self.wood.as_dyn(),
    }
  }

  pub const fn get_mut(&mut self, id: ResourceId) -> &mut dyn Resource {
    match id {
      ResourceId::Food => &mut self.food,
      ResourceId::Iron => &mut self.iron,
      ResourceId::Stone => &mut self.stone,
      ResourceId::Wood => &mut self.wood,
    }
  }

  #[inline]
  pub fn set(&mut self, id: ResourceId, value: u32) {
    self.get_mut(id).set(value);
  }

  /// Adds resources, respecting the storage capacity.
  pub const fn add_within_capacity(
    &mut self,
    diff: ResourcesDiff,
    capacity: OverallStorageCapacity,
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
  pub const fn checked_sub(&self, rhs: Resources) -> Option<Self> {
    Some(Self {
      food: self.food.checked_sub(rhs.food)?,
      iron: self.iron.checked_sub(rhs.iron)?,
      stone: self.stone.checked_sub(rhs.stone)?,
      wood: self.wood.checked_sub(rhs.wood)?,
    })
  }

  pub gen fn iter(&self) -> &dyn Resource {
    let Self { food, iron, stone, wood } = self;
    yield food.as_dyn();
    yield iron.as_dyn();
    yield stone.as_dyn();
    yield wood.as_dyn();
  }

  pub gen fn iter_mut(&mut self) -> &mut dyn Resource {
    let Self { food, iron, stone, wood } = self;
    yield food.as_dyn_mut();
    yield iron.as_dyn_mut();
    yield stone.as_dyn_mut();
    yield wood.as_dyn_mut();
  }

  /// Returns the total amount of resources, ignoring their type.
  #[inline]
  pub fn sum(&self) -> u32 {
    self.iter().map(Resource::as_u32).sum()
  }

  /// Returns the total amount of resources in the silo, ignoring their type.
  #[inline]
  pub fn sum_silo(&self) -> u32 {
    self.silo().sum()
  }

  /// Returns the total amount of resources in the warehouse, ignoring their type.
  #[inline]
  pub fn sum_warehouse(&self) -> u32 {
    self.warehouse().sum()
  }
}

const impl Default for Resources {
  fn default() -> Self {
    Self::new()
  }
}

const impl From<u32> for Resources {
  fn from(value: u32) -> Self {
    Self::splat(value)
  }
}

const impl Add for Resources {
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

const impl AddAssign for Resources {
  fn add_assign(&mut self, rhs: Self) {
    *self = Self {
      food: self.food + rhs.food,
      iron: self.iron + rhs.iron,
      stone: self.stone + rhs.stone,
      wood: self.wood + rhs.wood,
    };
  }
}

const impl Sub for Resources {
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

const impl SubAssign for Resources {
  fn sub_assign(&mut self, rhs: Self) {
    *self = Self {
      food: self.food - rhs.food,
      iron: self.iron - rhs.iron,
      stone: self.stone - rhs.stone,
      wood: self.wood - rhs.wood,
    };
  }
}

const impl Mul<u32> for Resources {
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

const impl Mul<NonZeroU32> for Resources {
  type Output = Resources;

  fn mul(self, rhs: NonZeroU32) -> Self::Output {
    self * rhs.get()
  }
}

const impl Mul<MarketFee> for Resources {
  type Output = Resources;

  fn mul(self, rhs: MarketFee) -> Self::Output {
    Self {
      food: self.food * rhs,
      iron: self.iron * rhs,
      stone: self.stone * rhs,
      wood: self.wood * rhs,
    }
  }
}

impl Sum for Resources {
  fn sum<I>(iter: I) -> Self
  where
    I: Iterator<Item = Self>,
  {
    iter.fold(Self::default(), |acc, resources| acc + resources)
  }
}

impl Sum<Resources> for u32 {
  fn sum<I>(iter: I) -> Self
  where
    I: Iterator<Item = Resources>,
  {
    iter.fold(0u32, |acc, resources| acc.saturating_add(resources.sum()))
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

          pub const ID: ResourceId = ResourceId::$resource;
          pub const MARKET_PRICE: Gold = Gold::new(1);

          #[inline]
          pub const fn new(value: u32) -> Self {
            Self(value)
          }

          #[inline]
          pub const fn as_dyn(&self) -> &dyn Resource {
            self
          }

          #[inline]
          pub const fn as_dyn_mut(&mut self) -> &mut dyn Resource {
            self
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

        impl Resource for $resource {
          fn id(&self) -> ResourceId {
            Self::ID
          }

          fn market_price(&self) -> Gold {
            Self::MARKET_PRICE
          }

          fn as_u32(&self) -> u32 {
            self.0
          }

          fn set(&mut self, value: u32) {
            *self = Self::new(value);
          }
        }

        const impl From<u32> for $resource {
          fn from(value: u32) -> Self {
            Self::new(value)
          }
        }

        const impl From<$resource> for u32 {
          fn from(value: $resource) -> Self {
            value.0
          }
        }

        const impl From<f64> for $resource {
          fn from(value: f64) -> Self {
            debug_assert!(value.is_finite());
            debug_assert!(value >= 0.0);
            Self(value.trunc() as u32)
          }
        }

        const impl From<$resource> for f64 {
          fn from(value: $resource) -> Self {
            f64::from(value.0)
          }
        }

        const impl From<MineProduction> for $resource {
          fn from(value: MineProduction) -> Self {
            Self(*value)
          }
        }

        const impl From<StorageCapacity> for $resource {
          fn from(value: StorageCapacity) -> Self {
            Self(*value)
          }
        }

        const impl From<$resource> for Resources {
          fn from(value: $resource) -> Self {
            let mut resources = Resources::new();
            resources.[<$resource:snake>] = value;
            resources
          }
        }

        const impl From<$resource> for Gold {
          fn from(value: $resource) -> Self {
            $resource::MARKET_PRICE * value.0
          }
        }

        const impl From<Gold> for $resource {
          fn from(value: Gold) -> Self {
            debug_assert!($resource::MARKET_PRICE > 0u32);
            let resource = value / $resource::MARKET_PRICE;
            Self::new(u32::from(resource))
          }
        }

        impl TryFrom<$resource> for i32 {
          type Error = $crate::error::Error;

          fn try_from(value: $resource) -> Result<Self> {
            match i32::try_from(value.0) {
              Ok(value) => Ok(value),
              Err(_) =>  {
                let resources = Resources::from(value);
                Err(Error::TooManyResources(resources))
              },
            }
          }
        }

        const impl PartialEq<u32> for $resource {
          fn eq(&self, other: &u32) -> bool {
            self.0.eq(other)
          }
        }

        const impl PartialOrd<u32> for $resource {
          fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
            self.0.partial_cmp(other)
          }
        }

        const impl Add for $resource {
          type Output = Self;

          fn add(self, rhs: Self) -> Self {
            Self(self.0.saturating_add(rhs.0))
          }
        }

        const impl Add<u32> for $resource {
          type Output = Self;

          fn add(self, rhs: u32) -> Self {
            Self(self.0.saturating_add(rhs))
          }
        }

        const impl AddAssign for $resource {
          fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
          }
        }

        const impl Sub for $resource {
          type Output = Self;

          fn sub(self, rhs: Self) -> Self {
            Self(self.0.saturating_sub(rhs.0))
          }
        }

        const impl Sub<u32> for $resource {
          type Output = Self;

          fn sub(self, rhs: u32) -> Self {
            Self(self.0.saturating_sub(rhs))
          }
        }

        const impl SubAssign for $resource {
          fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs;
          }
        }

        const impl Mul<u32> for $resource {
          type Output = Self;

          fn mul(self, rhs: u32) -> Self::Output {
            Self(self.0.saturating_mul(rhs))
          }
        }

        const impl Mul<NonZeroU32> for $resource {
          type Output = Self;

          fn mul(self, rhs: NonZeroU32) -> Self::Output {
            self * rhs.get()
          }
        }

        const impl Mul<MarketFee> for $resource {
          type Output = Self;

          fn mul(self, rhs: MarketFee) -> Self::Output {
            Self::from(self.mul_ceil(*rhs))
          }
        }

        const impl Mul<Stability> for $resource {
          type Output = $resource;

          fn mul(self, rhs: Stability) -> Self::Output {
            Self::from(self.mul_ceil(*rhs))
          }
        }

        const impl MulAssign<u32> for $resource {
          fn mul_assign(&mut self, rhs: u32) {
            *self = *self * rhs;
          }
        }

        const impl MulAssign<MarketFee> for $resource {
          fn mul_assign(&mut self, rhs: MarketFee) {
            *self = *self * rhs;
          }
        }

        const impl MulAssign<Stability> for $resource {
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

pub impl(crate) trait Resource: Send + Sync {
  fn id(&self) -> ResourceId;
  fn market_price(&self) -> Gold;

  fn set(&mut self, value: u32);
  fn clamp(&mut self, min: u32, max: u32) {
    let value = self.as_u32();
    self.set(value.clamp(min, max));
  }

  fn as_u32(&self) -> u32;
  fn as_f64(&self) -> f64 {
    f64::from(self.as_u32())
  }

  fn as_resources(&self) -> Resources
  where
    Self: Sized,
  {
    Resources::with_resource(self)
  }

  fn is_silo_resource(&self) -> bool {
    self.id().is_silo_resource()
  }

  fn is_warehouse_resource(&self) -> bool {
    self.id().is_warehouse_resource()
  }
}

#[subenum(SiloResourceId, WarehouseResourceId)]
#[derive(Copy, Debug, strum::Display, EnumIs, EnumIter, Hash, Deserialize, Serialize)]
#[derive_const(Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub enum ResourceId {
  #[subenum(SiloResourceId)]
  Food,

  #[subenum(WarehouseResourceId)]
  Iron,

  #[subenum(WarehouseResourceId)]
  Stone,

  #[subenum(WarehouseResourceId)]
  Wood,
}

impl ResourceId {
  pub fn is_silo_resource(self) -> bool {
    SiloResourceId::try_from(self).is_ok()
  }

  pub fn is_warehouse_resource(self) -> bool {
    WarehouseResourceId::try_from(self).is_ok()
  }
}
