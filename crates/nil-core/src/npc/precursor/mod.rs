// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod a;
mod b;

use crate::continent::{ContinentSize, Coord, Distance};
use crate::ethic::Ethics;
use crate::military::army::personnel::ArmyPersonnel;
use crate::resources::Resources;
use crate::resources::influence::Influence;
use derive_more::Deref;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

pub use crate::npc::precursor::a::A;
pub use crate::npc::precursor::b::B;

pub const trait Precursor: Send + Sync {
  fn id(&self) -> PrecursorId;
  fn ethics(&self) -> &Ethics;
  fn origin(&self) -> Coord;
  fn resources(&self) -> &Resources;
  fn resources_mut(&mut self) -> &mut Resources;
  fn influence(&self) -> Influence;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrecursorManager {
  a: A,
  b: B,
}

impl PrecursorManager {
  pub const fn new(size: ContinentSize) -> Self {
    Self { a: A::new(size), b: B::new(size) }
  }

  pub const fn precursor(&self, id: PrecursorId) -> &dyn Precursor {
    match id {
      PrecursorId::A => &self.a,
      PrecursorId::B => &self.b,
    }
  }

  pub(crate) const fn precursor_mut(&mut self, id: PrecursorId) -> &mut dyn Precursor {
    match id {
      PrecursorId::A => &mut self.a,
      PrecursorId::B => &mut self.b,
    }
  }

  pub fn precursors(&self) -> impl Iterator<Item = &dyn Precursor> {
    PrecursorId::iter().map(|id| self.precursor(id))
  }

  pub fn is_within_territory(&self, coord: Coord, size: ContinentSize) -> bool {
    let distance = initial_territory_radius(size);
    macro_rules! check {
      ($($id:ident),+ $(,)?) => {
        $(
          let precursor = self.precursor(PrecursorId::$id);
          if precursor.origin().is_within_distance(coord, distance) {
            return true;
          }
        )+
      };
    }

    check!(A, B);

    false
  }
}

#[derive(Deref)]
pub struct PrecursorBox(Box<dyn Precursor>);

impl PrecursorBox {
  #[inline]
  pub fn new(precursor: Box<dyn Precursor>) -> Self {
    Self(precursor)
  }

  #[inline]
  pub fn as_dyn(&self) -> &dyn Precursor {
    &*self.0
  }
}

impl<T> From<T> for PrecursorBox
where
  T: Precursor + 'static,
{
  fn from(value: T) -> Self {
    Self::new(Box::new(value))
  }
}

#[derive(Copy, Debug, Display, Hash, EnumIter, EnumString, Deserialize, Serialize)]
#[derive_const(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum PrecursorId {
  #[serde(rename = "A")]
  #[strum(serialize = "A")]
  A,

  #[serde(rename = "B")]
  #[strum(serialize = "B")]
  B,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct PublicPrecursor {
  id: PrecursorId,
  origin: Coord,
}

impl PublicPrecursor {
  pub const fn new<T>(precursor: &T) -> Self
  where
    T: [const] Precursor + ?Sized,
  {
    Self {
      id: precursor.id(),
      origin: precursor.origin(),
    }
  }
}

impl From<&dyn Precursor> for PublicPrecursor {
  fn from(precursor: &dyn Precursor) -> Self {
    Self::new(precursor)
  }
}

impl<T> const From<&T> for PublicPrecursor
where
  T: [const] Precursor,
{
  fn from(precursor: &T) -> Self {
    Self::new(precursor)
  }
}

/// Precursors start with an initial territory equal to one-tenth of the continent size.
/// In other words, they would begin with a 10×10 territory on a 100×100 continent.
#[inline]
pub const fn initial_territory_radius(size: ContinentSize) -> Distance {
  Distance::new(size.get().div_ceil(20).next_multiple_of(2))
}

#[inline]
pub const fn initial_city_amount(size: ContinentSize) -> u8 {
  size.get().div_ceil(10).saturating_mul(2)
}

pub fn initial_offensive_personnel() -> ArmyPersonnel {
  ArmyPersonnel::builder()
    .axeman(5000)
    .light_cavalry(2500)
    .ram(300)
    .build()
}

pub fn initial_defensive_personnel() -> ArmyPersonnel {
  ArmyPersonnel::builder()
    .archer(3000)
    .pikeman(5000)
    .swordsman(5000)
    .heavy_cavalry(1000)
    .build()
}
