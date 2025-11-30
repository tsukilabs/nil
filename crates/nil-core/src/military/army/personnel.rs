// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::military::squad::{Squad, SquadSize};
use crate::military::unit::stats::speed::Speed;
use crate::military::unit::{UnitId, UnitIdIter};
use crate::ranking::Score;
use crate::resources::Maintenance;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub, SubAssign};
use strum::IntoEnumIterator;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ArmyPersonnel {
  archer: Squad,
  axeman: Squad,
  heavy_cavalry: Squad,
  light_cavalry: Squad,
  pikeman: Squad,
  swordsman: Squad,
}

#[bon::bon]
impl ArmyPersonnel {
  #[builder]
  pub fn new(
    #[builder(default, into)] archer: SquadSize,
    #[builder(default, into)] axeman: SquadSize,
    #[builder(default, into)] heavy_cavalry: SquadSize,
    #[builder(default, into)] light_cavalry: SquadSize,
    #[builder(default, into)] pikeman: SquadSize,
    #[builder(default, into)] swordsman: SquadSize,
  ) -> Self {
    use UnitId::*;
    Self {
      archer: Squad::new(Archer, archer),
      axeman: Squad::new(Axeman, axeman),
      heavy_cavalry: Squad::new(HeavyCavalry, heavy_cavalry),
      light_cavalry: Squad::new(LightCavalry, light_cavalry),
      pikeman: Squad::new(Pikeman, pikeman),
      swordsman: Squad::new(Swordsman, swordsman),
    }
  }

  pub fn squad(&self, id: UnitId) -> &Squad {
    match id {
      UnitId::Archer => &self.archer,
      UnitId::Axeman => &self.axeman,
      UnitId::HeavyCavalry => &self.heavy_cavalry,
      UnitId::LightCavalry => &self.light_cavalry,
      UnitId::Pikeman => &self.pikeman,
      UnitId::Swordsman => &self.swordsman,
    }
  }

  fn squad_mut(&mut self, id: UnitId) -> &mut Squad {
    match id {
      UnitId::Archer => &mut self.archer,
      UnitId::Axeman => &mut self.axeman,
      UnitId::HeavyCavalry => &mut self.heavy_cavalry,
      UnitId::LightCavalry => &mut self.light_cavalry,
      UnitId::Pikeman => &mut self.pikeman,
      UnitId::Swordsman => &mut self.swordsman,
    }
  }

  #[inline]
  pub fn iter(&self) -> ArmyPersonnelIter<'_> {
    ArmyPersonnelIter::new(self)
  }

  pub fn speed(&self) -> Option<Speed> {
    self
      .iter()
      .filter(|squad| squad.size() > 0u32)
      .map(|squad| squad.unit().speed())
      .min_by(|a, b| a.total_cmp(b))
  }

  pub fn score(&self) -> Score {
    self
      .iter()
      .fold(Score::default(), |mut score, squad| {
        score += squad.score();
        score
      })
  }

  pub fn maintenance(&self) -> Maintenance {
    self
      .iter()
      .fold(Maintenance::default(), |mut maintenance, squad| {
        maintenance += squad.maintenance();
        maintenance
      })
  }
}

impl Default for ArmyPersonnel {
  fn default() -> Self {
    Self::builder().build()
  }
}

impl FromIterator<Squad> for ArmyPersonnel {
  fn from_iter<T>(iter: T) -> Self
  where
    T: IntoIterator<Item = Squad>,
  {
    iter
      .into_iter()
      .fold(Self::default(), |mut personnel, squad| {
        personnel += squad;
        personnel
      })
  }
}

impl<'a> IntoIterator for &'a ArmyPersonnel {
  type Item = &'a Squad;
  type IntoIter = ArmyPersonnelIter<'a>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl Add for ArmyPersonnel {
  type Output = ArmyPersonnel;

  fn add(mut self, rhs: Self) -> Self::Output {
    self += rhs;
    self
  }
}

impl Add<Squad> for ArmyPersonnel {
  type Output = ArmyPersonnel;

  fn add(mut self, rhs: Squad) -> Self::Output {
    self += rhs;
    self
  }
}

impl AddAssign for ArmyPersonnel {
  fn add_assign(&mut self, rhs: Self) {
    for squad in &rhs {
      *self.squad_mut(squad.id()) += squad.size();
    }
  }
}

impl AddAssign<Squad> for ArmyPersonnel {
  fn add_assign(&mut self, rhs: Squad) {
    *self.squad_mut(rhs.id()) += rhs;
  }
}

impl Sub for ArmyPersonnel {
  type Output = ArmyPersonnel;

  fn sub(mut self, rhs: Self) -> Self::Output {
    self -= rhs;
    self
  }
}

impl SubAssign for ArmyPersonnel {
  fn sub_assign(&mut self, rhs: Self) {
    for squad in &rhs {
      *self.squad_mut(squad.id()) -= squad.size();
    }
  }
}

impl SubAssign<Squad> for ArmyPersonnel {
  fn sub_assign(&mut self, rhs: Squad) {
    *self.squad_mut(rhs.id()) -= rhs;
  }
}

pub struct ArmyPersonnelIter<'a> {
  personnel: &'a ArmyPersonnel,
  iter: UnitIdIter,
}

impl<'a> ArmyPersonnelIter<'a> {
  pub fn new(personnel: &'a ArmyPersonnel) -> Self {
    Self { personnel, iter: UnitId::iter() }
  }
}

impl<'a> Iterator for ArmyPersonnelIter<'a> {
  type Item = &'a Squad;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.personnel.squad(self.iter.next()?))
  }
}
