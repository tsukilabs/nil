// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod a;
mod b;

use crate::continent::{ContinentSize, Coord, Distance};
use crate::ethic::Ethics;
use crate::military::army::ArmyPersonnel;
use crate::resources::Resources;
use serde::{Deserialize, Serialize};
use strum::Display;

pub use crate::npc::precursor::a::A;
pub use crate::npc::precursor::b::B;

pub trait Precursor {
  fn id(&self) -> PrecursorId;
  fn ethics(&self) -> &Ethics;
  fn origin(&self) -> Coord;
  fn resources(&self) -> &Resources;
  fn resources_mut(&mut self) -> &mut Resources;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrecursorManager {
  a: A,
  b: B,
}

impl PrecursorManager {
  pub fn new(size: ContinentSize) -> Self {
    Self { a: A::new(size), b: B::new(size) }
  }

  pub(crate) fn precursor(&self, id: PrecursorId) -> &dyn Precursor {
    match id {
      PrecursorId::A => &self.a,
      PrecursorId::B => &self.b,
    }
  }

  pub(crate) fn precursor_mut(&mut self, id: PrecursorId) -> &mut dyn Precursor {
    match id {
      PrecursorId::A => &mut self.a,
      PrecursorId::B => &mut self.b,
    }
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

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum PrecursorId {
  #[serde(rename = "A")]
  #[strum(serialize = "A")]
  A,
  #[serde(rename = "B")]
  #[strum(serialize = "B")]
  B,
}

#[inline]
pub fn initial_territory_radius(size: ContinentSize) -> Distance {
  Distance::new(size.get().div_ceil(20).next_multiple_of(2))
}

#[inline]
pub fn initial_village_amount(size: ContinentSize) -> u8 {
  size.get().div_ceil(10).saturating_mul(2)
}

pub fn initial_offensive_personnel() -> ArmyPersonnel {
  ArmyPersonnel::builder()
    .axeman(5000)
    .light_cavalry(2500)
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
