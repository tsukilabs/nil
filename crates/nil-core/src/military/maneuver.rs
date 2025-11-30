// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::{Coord, Distance};
use crate::military::army::ArmyId;
use crate::military::unit::stats::speed::Speed;
use serde::{Deserialize, Serialize};
use std::ops::{Sub, SubAssign};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Maneuver {
  id: ManeuverId,
  army: ArmyId,
  kind: ManeuverKind,
  origin: Coord,
  destination: Coord,
  state: ManeuverState,
}

impl Maneuver {
  pub(super) fn new(request: ManeuverRequest) -> (ManeuverId, Self) {
    let id = ManeuverId::new();
    let distance = request.origin.distance(request.destination);

    let maneuver = Self {
      id,
      kind: request.kind,
      army: request.army,
      origin: request.origin,
      destination: request.destination,
      state: ManeuverState::new(distance.into()),
    };

    (id, maneuver)
  }

  #[inline]
  pub fn id(&self) -> ManeuverId {
    self.id
  }

  #[inline]
  pub fn kind(&self) -> ManeuverKind {
    self.kind
  }

  #[inline]
  pub fn origin(&self) -> Coord {
    self.origin
  }

  #[inline]
  pub fn destination(&self) -> Coord {
    self.destination
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

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ManeuverKind {
  Attack,
  Support,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ManeuverState {
  Done,
  Pending { distance: ManeuverDistance },
}

impl ManeuverState {
  fn new(distance: ManeuverDistance) -> Self {
    Self::Pending { distance }
  }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct ManeuverDistance(f64);

impl Sub for ManeuverDistance {
  type Output = ManeuverDistance;

  fn sub(mut self, rhs: Self) -> Self::Output {
    self -= rhs;
    self
  }
}

impl Sub<Speed> for ManeuverDistance {
  type Output = ManeuverDistance;

  fn sub(mut self, rhs: Speed) -> Self::Output {
    self -= rhs;
    self
  }
}

impl SubAssign for ManeuverDistance {
  fn sub_assign(&mut self, rhs: Self) {
    *self = Self(self.0 - rhs.0);
  }
}

impl SubAssign<Speed> for ManeuverDistance {
  fn sub_assign(&mut self, rhs: Speed) {
    *self = Self(self.0 - f64::from(rhs));
  }
}

impl From<Distance> for ManeuverDistance {
  fn from(distance: Distance) -> Self {
    Self(f64::from(distance))
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManeuverRequest {
  army: ArmyId,
  kind: ManeuverKind,
  origin: Coord,
  destination: Coord,
}
