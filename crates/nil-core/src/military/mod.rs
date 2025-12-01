// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod army;
pub mod maneuver;
pub mod squad;
pub mod unit;

use crate::continent::{ContinentIndex, ContinentKey, ContinentSize, Coord};
use crate::error::{Error, Result};
use crate::military::army::ArmyId;
use crate::military::maneuver::{ManeuverId, ManeuverRequest};
use crate::ranking::Score;
use crate::resources::Maintenance;
use crate::ruler::Ruler;
use army::{Army, ArmyPersonnel};
use maneuver::Maneuver;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Military {
  continent: HashMap<ContinentIndex, Vec<Army>>,
  continent_size: ContinentSize,
  maneuvers: HashMap<ManeuverId, Maneuver>,
}

// TODO: All armies should be reconciled when the round ends to avoid having
// multiple copies of idle armies owned by the same ruler in the same location.
impl Military {
  pub(crate) fn new(size: ContinentSize) -> Self {
    Self {
      continent: HashMap::new(),
      continent_size: size,
      maneuvers: HashMap::new(),
    }
  }

  pub(crate) fn spawn<K, R>(&mut self, key: K, owner: R, personnel: ArmyPersonnel)
  where
    K: ContinentKey,
    R: Into<Ruler>,
  {
    let index = key.into_index(self.continent_size);
    let army = Army::builder()
      .owner(owner)
      .personnel(personnel)
      .build();

    self
      .continent
      .entry(index)
      .or_default()
      .push(army);
  }

  /// Creates a new instance containing only entries related to a given set of coords.
  #[must_use]
  pub fn intersection<K, I>(&self, keys: I) -> Result<Self>
  where
    K: ContinentKey,
    I: IntoIterator<Item = K>,
  {
    let mut military = Self::new(self.continent_size);
    for key in keys {
      let coord = key.into_coord(self.continent_size)?;
      let index = coord.into_index(self.continent_size);
      if let Some(armies) = self.continent.get(&index).cloned() {
        military.continent.insert(index, armies);
      }

      let maneuvers = self
        .maneuvers_at(coord)
        .map(|maneuver| (maneuver.id(), maneuver.clone()));

      military.maneuvers.extend(maneuvers);
    }

    Ok(military)
  }

  pub fn army(&self, id: ArmyId) -> Result<&Army> {
    self
      .armies()
      .find(|army| army.id() == id)
      .ok_or(Error::ArmyNotFound(id))
  }

  pub(crate) fn army_mut(&mut self, id: ArmyId) -> Result<&mut Army> {
    self
      .armies_mut()
      .find(|army| army.id() == id)
      .ok_or(Error::ArmyNotFound(id))
  }

  pub fn armies(&self) -> impl Iterator<Item = &Army> {
    self.continent.values().flatten()
  }

  pub(crate) fn armies_mut(&mut self) -> impl Iterator<Item = &mut Army> {
    self.continent.values_mut().flatten()
  }

  pub fn armies_at<K>(&self, key: K) -> &[Army]
  where
    K: ContinentKey,
  {
    let index = key.into_index(self.continent_size);
    self
      .continent
      .get(&index)
      .map(Vec::as_slice)
      .unwrap_or_default()
  }

  pub fn idle_armies_at<K>(&self, key: K) -> impl Iterator<Item = &Army>
  where
    K: ContinentKey,
  {
    self
      .armies_at(key)
      .iter()
      .filter(|army| army.is_idle())
  }

  pub fn armies_of<R>(&self, owner: R) -> impl Iterator<Item = &Army>
  where
    R: Into<Ruler>,
  {
    let owner: Ruler = owner.into();
    self
      .continent
      .values()
      .flatten()
      .filter(move |army| army.owner() == &owner)
  }

  pub fn score_of<R>(&self, owner: R) -> Score
  where
    R: Into<Ruler>,
  {
    self
      .armies_of(owner)
      .fold(Score::default(), |mut score, army| {
        score += army.score();
        score
      })
  }

  pub fn maintenance_of<R>(&self, owner: R) -> Maintenance
  where
    R: Into<Ruler>,
  {
    self
      .armies_of(owner)
      .fold(Maintenance::default(), |mut maintenance, army| {
        maintenance += army.maintenance();
        maintenance
      })
  }

  #[inline]
  pub fn maneuver(&self, id: ManeuverId) -> Result<&Maneuver> {
    self
      .maneuvers
      .get(&id)
      .ok_or(Error::ManeuverNotFound(id))
  }

  pub fn maneuvers(&self) -> impl Iterator<Item = &Maneuver> {
    self.maneuvers.values()
  }

  /// Find all maneuvers whose origin or destination matches the specified coord.
  pub fn maneuvers_at(&self, coord: Coord) -> impl Iterator<Item = &Maneuver> {
    self
      .maneuvers()
      .filter(move |maneuver| maneuver.matches_coord(coord))
  }

  pub(crate) fn request_maneuver(&mut self, request: &ManeuverRequest) -> Result<()> {
    let army = self.army_mut(request.army)?;
    if army.is_idle() {
      let (id, maneuver) = Maneuver::new(request)?;
      army.set_maneuver(id);
      self.maneuvers.insert(id, maneuver);
      Ok(())
    } else {
      Err(Error::ArmyNotIdle(request.army))
    }
  }
}
