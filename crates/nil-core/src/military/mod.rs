// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#[cfg(test)]
mod tests;

pub mod army;
pub mod maneuver;
pub mod squad;
pub mod unit;

use crate::continent::{ContinentIndex, ContinentKey, ContinentSize, Coord};
use crate::error::{Error, Result};
use crate::military::army::{ArmyId, collapse_armies};
use crate::military::maneuver::ManeuverId;
use crate::military::squad::Squad;
use crate::ranking::Score;
use crate::resources::Maintenance;
use crate::ruler::Ruler;
use army::{Army, ArmyPersonnel};
use itertools::Itertools;
use maneuver::Maneuver;
use nil_util::result::WrapOk;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Military {
  continent: HashMap<ContinentIndex, Vec<Army>>,
  continent_size: ContinentSize,
  maneuvers: HashMap<ManeuverId, Maneuver>,
}

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
    let ruler: Ruler = owner.into();
    let army = Army::builder()
      .owner(ruler)
      .personnel(personnel)
      .build();

    let index = key.into_index(self.continent_size);
    self
      .continent
      .entry(index)
      .or_default()
      .push(army);

    self.collapse_armies_in(index);
  }

  pub fn collapse_armies(&mut self) {
    self
      .continent
      .values_mut()
      .for_each(collapse_armies);

    self
      .continent
      .retain(|_, armies| !armies.is_empty());
  }

  pub fn collapse_armies_in<K>(&mut self, key: K)
  where
    K: ContinentKey,
  {
    let index = key.into_index(self.continent_size);
    if let Some(armies) = self.continent.get_mut(&index) {
      collapse_armies(armies);
    }
  }

  /// Creates a new instance containing only entries related to a given set of coords.
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

  pub(crate) fn remove_army(&mut self, id: ArmyId) -> Result<Army> {
    let (curr_vec, pos) = self
      .continent
      .values_mut()
      .find_map(|armies| {
        armies
          .iter()
          .position(|army| army.id() == id)
          .map(|pos| (armies, pos))
      })
      .ok_or(Error::ArmyNotFound(id))?;

    Ok(curr_vec.swap_remove(pos))
  }

  pub(crate) fn relocate_army<K>(&mut self, id: ArmyId, new_key: K) -> Result<()>
  where
    K: ContinentKey,
  {
    let army = self.remove_army(id)?;
    let index = new_key.into_index(self.continent_size);
    self
      .continent
      .entry(index)
      .or_default()
      .push(army);

    self.collapse_armies_in(index);

    Ok(())
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

  #[inline]
  pub fn count_armies(&self) -> usize {
    self.armies().count()
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

  pub(crate) fn armies_mut_at<K>(&mut self, key: K) -> &mut [Army]
  where
    K: ContinentKey,
  {
    let index = key.into_index(self.continent_size);
    self
      .continent
      .get_mut(&index)
      .map(Vec::as_mut_slice)
      .unwrap_or_default()
  }

  pub fn count_armies_at<K>(&self, key: K) -> usize
  where
    K: ContinentKey,
  {
    self.armies_at(key).iter().count()
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
      .filter(move |army| army.is_owned_by(&owner))
  }

  #[inline]
  pub fn personnel(&self, id: ArmyId) -> Result<&ArmyPersonnel> {
    self.army(id).map(Army::personnel)
  }

  #[inline]
  pub fn squads(&self, id: ArmyId) -> Result<Vec<Squad>> {
    self
      .personnel(id)
      .cloned()
      .map(ArmyPersonnel::to_vec)
  }

  pub fn fold_idle_personnel_at<K>(&self, key: K) -> ArmyPersonnel
  where
    K: ContinentKey,
  {
    self
      .idle_armies_at(key)
      .map(Army::personnel)
      .fold(ArmyPersonnel::default(), |mut acc, personnel| {
        acc += personnel;
        acc
      })
  }

  pub fn idle_squads_at<K>(&self, key: K) -> Vec<Squad>
  where
    K: ContinentKey,
  {
    self.fold_idle_personnel_at(key).to_vec()
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

  pub(crate) fn insert_maneuver(&mut self, maneuver: Maneuver) {
    self
      .maneuvers
      .insert(maneuver.id(), maneuver);
  }

  pub(crate) fn advance_maneuvers(&mut self) -> Result<Vec<Maneuver>> {
    let mut done = Vec::new();
    for (id, maneuver) in &mut self.maneuvers {
      maneuver.advance()?;
      if maneuver.is_done() {
        done.push(*id);
      }
    }

    done
      .into_iter()
      .filter_map(|id| self.maneuvers.remove(&id))
      .collect_vec()
      .wrap_ok()
  }
}
