// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::military::squad::{Squad, SquadSize};
use crate::military::unit::stats::haul::Haul;
use crate::military::unit::stats::speed::Speed;
use crate::military::unit::{UnitId, UnitIdIter};
use crate::ranking::Score;
use crate::resources::maintenance::Maintenance;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub, SubAssign};
use strum::IntoEnumIterator;
use tap::Pipe;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ArmyPersonnel {
  archer: Squad,
  axeman: Squad,
  heavy_cavalry: Squad,
  light_cavalry: Squad,
  pikeman: Squad,
  ram: Squad,
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
    #[builder(default, into)] ram: SquadSize,
    #[builder(default, into)] swordsman: SquadSize,
  ) -> Self {
    use UnitId::*;
    Self {
      archer: Squad::new(Archer, archer),
      axeman: Squad::new(Axeman, axeman),
      heavy_cavalry: Squad::new(HeavyCavalry, heavy_cavalry),
      light_cavalry: Squad::new(LightCavalry, light_cavalry),
      pikeman: Squad::new(Pikeman, pikeman),
      ram: Squad::new(Ram, ram),
      swordsman: Squad::new(Swordsman, swordsman),
    }
  }

  pub fn to_vec(self) -> Vec<Squad> {
    Vec::<Squad>::from(self)
  }

  #[inline]
  pub fn iter(&self) -> ArmyPersonnelIter<'_> {
    ArmyPersonnelIter::new(self)
  }

  pub fn slowest_squad(&self) -> Option<&Squad> {
    self
      .iter()
      .filter(|squad| squad.size() > 0u32)
      .min_by(|a, b| a.speed().total_cmp(&b.speed()))
  }

  pub fn speed(&self) -> Speed {
    self
      .slowest_squad()
      .map(Squad::speed)
      .unwrap_or_default()
  }

  pub fn haul(&self) -> Haul {
    self
      .iter()
      .fold(Haul::default(), |mut haul, squad| {
        haul += squad.haul();
        haul
      })
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
}

macro_rules! impl_army_personnel {
  ($($unit:ident),+) => {
    paste::paste! {
      impl ArmyPersonnel {
        pub fn splat(size: impl Into<SquadSize>) -> Self {
          let size: SquadSize = size.into();
          Self::builder()
            $(.[<$unit:snake>](size))+
            .build()
        }

        pub fn random() -> Self {
          Self::builder()
            $(.[<$unit:snake>](SquadSize::random()))+
            .build()
        }

        pub fn squad(&self, id: UnitId) -> &Squad {
          match id {
            $(UnitId::$unit => &self.[<$unit:snake>],)+
          }
        }

        fn squad_mut(&mut self, id: UnitId) -> &mut Squad {
          match id {
            $(UnitId::$unit => &mut self.[<$unit:snake>],)+
          }
        }

        $(
          #[inline]
          pub fn [<$unit:snake>](&self) -> &Squad {
            &self.[<$unit:snake>]
          }
        )+

        pub fn has_enough_personnel(&self, required: &ArmyPersonnel) -> bool {
          $(self.[<$unit:snake>].size() >= required.[<$unit:snake>].size() && )+ true
        }

        pub fn checked_sub(&self, rhs: &Self) -> Option<Self> {
          Self::builder()
            $(
              .[<$unit:snake>](
                self
                  .[<$unit:snake>]
                  .size()
                  .checked_sub(rhs.[<$unit:snake>].size())?
              )
            )+
            .build()
            .pipe(Some)
        }
      }

      impl From<ArmyPersonnel> for Vec<Squad> {
        fn from(personnel: ArmyPersonnel) -> Self {
          vec![$(personnel.[<$unit:snake>],)+]
        }
      }
    }
  };
}

impl_army_personnel!(
  Archer,
  Axeman,
  HeavyCavalry,
  LightCavalry,
  Pikeman,
  Ram,
  Swordsman
);

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
