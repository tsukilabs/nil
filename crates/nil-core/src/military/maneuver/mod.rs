// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod distance;

use crate::continent::Coord;
use crate::error::{Error, Result};
use crate::military::army::{ArmyId, ArmyPersonnel};
use crate::military::unit::stats::speed::Speed;
use crate::resources::Resources;
use crate::ruler::Ruler;
use bon::Builder;
use serde::{Deserialize, Serialize};
use strum::EnumIs;
use uuid::Uuid;

pub use distance::ManeuverDistance;

#[must_use]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Maneuver {
  id: ManeuverId,
  origin: Coord,
  destination: Coord,
  army: ArmyId,
  kind: ManeuverKind,
  direction: ManeuverDirection,
  state: ManeuverState,
  speed: Speed,
  hauled_resources: Option<ManeuverHaul>,
}

#[bon::bon]
impl Maneuver {
  #[builder]
  pub(crate) fn new(
    army: ArmyId,
    kind: ManeuverKind,
    origin: Coord,
    destination: Coord,
    speed: Speed,
  ) -> Result<(ManeuverId, Self)> {
    let distance = origin.distance(destination);
    if origin == destination || distance == 0u8 {
      return Err(Error::OriginIsDestination(origin));
    }

    let id = ManeuverId::new();
    let state = ManeuverState::with_distance(distance.into());
    let maneuver = Self {
      id,
      origin,
      destination,
      army,
      kind,
      direction: ManeuverDirection::Going,
      state,
      speed,
      hauled_resources: None,
    };

    Ok((id, maneuver))
  }

  pub(super) fn advance(&mut self) -> Result<()> {
    let is_done = match &mut self.state {
      ManeuverState::Done => {
        return Err(Error::ManeuverIsDone(self.id));
      }
      ManeuverState::Pending { distance } => {
        *distance -= self.speed;
        debug_assert!(distance.is_finite());
        *distance <= 0.0
      }
    };

    if is_done {
      self.state = ManeuverState::Done;
    }

    Ok(())
  }

  pub(crate) fn reverse(&mut self) -> Result<()> {
    if self.is_pending() {
      return Err(Error::ManeuverIsPending(self.id));
    } else if self.is_returning() {
      return Err(Error::ManeuverIsReturning(self.id));
    }

    let distance = self.origin.distance(self.destination);
    self.state = ManeuverState::with_distance(distance.into());
    self.direction = ManeuverDirection::Returning;

    Ok(())
  }

  #[inline]
  pub fn id(&self) -> ManeuverId {
    self.id
  }

  #[inline]
  pub fn origin(&self) -> Coord {
    self.origin
  }

  #[inline]
  pub fn destination(&self) -> Coord {
    self.destination
  }

  #[inline]
  pub fn army(&self) -> ArmyId {
    self.army
  }

  #[inline]
  pub fn kind(&self) -> ManeuverKind {
    self.kind
  }

  #[inline]
  pub fn direction(&self) -> ManeuverDirection {
    self.direction
  }

  #[inline]
  pub fn state(&self) -> &ManeuverState {
    &self.state
  }

  #[inline]
  pub fn speed(&self) -> Speed {
    self.speed
  }

  #[inline]
  pub fn hauled_resources(&self) -> Option<&ManeuverHaul> {
    self.hauled_resources.as_ref()
  }

  #[inline]
  pub(crate) fn hauled_resources_mut(&mut self) -> &mut Option<ManeuverHaul> {
    &mut self.hauled_resources
  }

  #[inline]
  pub fn is_done(&self) -> bool {
    self.state.is_done()
  }

  #[inline]
  pub fn is_pending(&self) -> bool {
    self.state.is_pending()
  }

  #[inline]
  pub fn is_going(&self) -> bool {
    self.direction.is_going()
  }

  #[inline]
  pub fn is_returning(&self) -> bool {
    self.direction.is_returning()
  }

  /// Checks whether the maneuver's origin or destination matches the coord.
  #[inline]
  pub fn matches_coord(&self, coord: Coord) -> bool {
    coord == self.origin || coord == self.destination
  }
}

#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct ManeuverId(Uuid);

impl ManeuverId {
  pub fn new() -> Self {
    Self(Uuid::now_v7())
  }
}

impl Default for ManeuverId {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, EnumIs)]
#[serde(rename_all = "kebab-case")]
pub enum ManeuverKind {
  Attack,
  Support,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, EnumIs)]
#[serde(rename_all = "kebab-case")]
pub enum ManeuverDirection {
  Going,
  Returning,
}

#[derive(Clone, Debug, Deserialize, Serialize, EnumIs)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ManeuverState {
  Done,
  Pending { distance: ManeuverDistance },
}

impl ManeuverState {
  fn with_distance(distance: ManeuverDistance) -> Self {
    Self::Pending { distance }
  }
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManeuverHaul {
  ruler: Ruler,
  resources: Resources,
}

impl ManeuverHaul {
  #[inline]
  pub fn ruler(&self) -> &Ruler {
    &self.ruler
  }

  #[inline]
  pub fn resources(&self) -> &Resources {
    &self.resources
  }
}

impl From<ManeuverHaul> for Resources {
  fn from(haul: ManeuverHaul) -> Self {
    haul.resources
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManeuverRequest {
  pub kind: ManeuverKind,
  pub origin: Coord,
  pub destination: Coord,
  pub personnel: ArmyPersonnel,
}
