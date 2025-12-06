// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod distance;

use crate::continent::Coord;
use crate::error::{Error, Result};
use crate::military::army::{ArmyId, ArmyPersonnel};
use crate::military::unit::stats::speed::Speed;
use serde::{Deserialize, Serialize};
use strum::EnumIs;
use uuid::Uuid;

pub use distance::ManeuverDistance;

#[must_use]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Maneuver {
  id: ManeuverId,
  army: ArmyId,
  kind: ManeuverKind,
  direction: ManeuverDirection,
  origin: Coord,
  destination: Coord,
  state: ManeuverState,
  speed: Speed,
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
      army,
      kind,
      direction: ManeuverDirection::Going,
      origin,
      destination,
      state,
      speed,
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
  pub fn origin(&self) -> Coord {
    self.origin
  }

  #[inline]
  pub fn destination(&self) -> Coord {
    self.destination
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ManeuverId(Uuid);

impl ManeuverId {
  pub fn new() -> Self {
    Self(Uuid::new_v4())
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManeuverRequest {
  pub kind: ManeuverKind,
  pub origin: Coord,
  pub destination: Coord,
  pub personnel: ArmyPersonnel,
}
