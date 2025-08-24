// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::military::squad::{Squad, SquadSize};
use crate::military::unit::UnitId;
use crate::ranking::Score;
use crate::resources::Maintenance;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Debug, Deserialize, Serialize)]
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

  pub fn score(&self) -> Score {
    let mut score = Score::default();
    score += self.archer.score();
    score += self.axeman.score();
    score += self.heavy_cavalry.score();
    score += self.light_cavalry.score();
    score += self.pikeman.score();
    score += self.swordsman.score();
    score
  }

  pub fn maintenance(&self) -> Maintenance {
    let mut maintenance = Maintenance::default();
    maintenance += self.archer.maintenance();
    maintenance += self.axeman.maintenance();
    maintenance += self.heavy_cavalry.maintenance();
    maintenance += self.light_cavalry.maintenance();
    maintenance += self.pikeman.maintenance();
    maintenance += self.swordsman.maintenance();
    maintenance
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
    self.archer += rhs.archer;
    self.axeman += rhs.axeman;
    self.heavy_cavalry += rhs.heavy_cavalry;
    self.light_cavalry += rhs.light_cavalry;
    self.pikeman += rhs.pikeman;
    self.swordsman += rhs.swordsman;
  }
}

impl AddAssign<Squad> for ArmyPersonnel {
  fn add_assign(&mut self, rhs: Squad) {
    match rhs.id() {
      UnitId::Archer => self.archer += rhs,
      UnitId::Axeman => self.axeman += rhs,
      UnitId::HeavyCavalry => self.heavy_cavalry += rhs,
      UnitId::LightCavalry => self.light_cavalry += rhs,
      UnitId::Pikeman => self.pikeman += rhs,
      UnitId::Swordsman => self.swordsman += rhs,
    }
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
    self.archer -= rhs.archer;
    self.axeman -= rhs.axeman;
    self.heavy_cavalry -= rhs.heavy_cavalry;
    self.light_cavalry -= rhs.light_cavalry;
    self.pikeman -= rhs.pikeman;
    self.swordsman -= rhs.swordsman;
  }
}

impl SubAssign<Squad> for ArmyPersonnel {
  fn sub_assign(&mut self, rhs: Squad) {
    match rhs.id() {
      UnitId::Archer => self.archer -= rhs,
      UnitId::Axeman => self.axeman -= rhs,
      UnitId::HeavyCavalry => self.heavy_cavalry -= rhs,
      UnitId::LightCavalry => self.light_cavalry -= rhs,
      UnitId::Pikeman => self.pikeman -= rhs,
      UnitId::Swordsman => self.swordsman -= rhs,
    }
  }
}
