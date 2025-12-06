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

  pub fn splat(size: impl Into<SquadSize>) -> Self {
    let size: SquadSize = size.into();
    Self::builder()
      .archer(size)
      .axeman(size)
      .heavy_cavalry(size)
      .light_cavalry(size)
      .pikeman(size)
      .swordsman(size)
      .build()
  }

  pub fn random() -> Self {
    Self::builder()
      .archer(SquadSize::random())
      .axeman(SquadSize::random())
      .heavy_cavalry(SquadSize::random())
      .light_cavalry(SquadSize::random())
      .pikeman(SquadSize::random())
      .swordsman(SquadSize::random())
      .build()
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

  pub fn to_vec(self) -> Vec<Squad> {
    Vec::<Squad>::from(self)
  }

  #[inline]
  pub fn iter(&self) -> ArmyPersonnelIter<'_> {
    ArmyPersonnelIter::new(self)
  }

  pub fn speed(&self) -> Speed {
    self
      .iter()
      .filter(|squad| squad.size() > 0u32)
      .map(Squad::speed)
      .min_by(|a, b| a.total_cmp(b))
      .unwrap_or_default()
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

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.iter().all(Squad::is_empty)
  }

  pub fn has_enough_personnel(&self, required: &ArmyPersonnel) -> bool {
    self.archer.size() >= required.archer.size()
      && self.axeman.size() >= required.axeman.size()
      && self.heavy_cavalry.size() >= required.heavy_cavalry.size()
      && self.light_cavalry.size() >= required.light_cavalry.size()
      && self.pikeman.size() >= required.pikeman.size()
      && self.swordsman.size() >= required.swordsman.size()
  }

  pub fn checked_sub(&self, rhs: &Self) -> Option<Self> {
    macro_rules! sub {
      ($unit:ident) => {{
        self
          .$unit
          .size()
          .checked_sub(rhs.$unit.size())
      }};
    }

    Some(
      Self::builder()
        .archer(sub!(archer)?)
        .axeman(sub!(axeman)?)
        .heavy_cavalry(sub!(heavy_cavalry)?)
        .light_cavalry(sub!(light_cavalry)?)
        .pikeman(sub!(pikeman)?)
        .swordsman(sub!(swordsman)?)
        .build(),
    )
  }
}

impl Default for ArmyPersonnel {
  fn default() -> Self {
    Self::builder().build()
  }
}

impl From<SquadSize> for ArmyPersonnel {
  fn from(size: SquadSize) -> Self {
    ArmyPersonnel::splat(size)
  }
}

impl From<ArmyPersonnel> for Vec<Squad> {
  fn from(personnel: ArmyPersonnel) -> Self {
    vec![
      personnel.archer,
      personnel.axeman,
      personnel.heavy_cavalry,
      personnel.light_cavalry,
      personnel.pikeman,
      personnel.swordsman,
    ]
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
    self += &rhs;
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
    *self += &rhs;
  }
}

impl AddAssign<&ArmyPersonnel> for ArmyPersonnel {
  fn add_assign(&mut self, rhs: &ArmyPersonnel) {
    for squad in rhs {
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
