// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::infrastructure::building::BuildingLevel;
use crate::village::Stability;
use derive_more::Deref;
use nil_num::impl_mul_ceil;
use nil_num::ops::MulCeil;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

/// Força de trabalho é um recurso especial usado para construir edifícios e recrutar tropas.
/// A quantidade gerada por turno será sempre igual ao nível do edifício relevante (ex.: prefeitura).
///
/// Ao contrário dos outros recursos, a força de trabalho jamais deve acumular para o próximo turno.
/// Tudo o que não for usado deve ser descartado.
#[derive(Clone, Copy, Debug, Deref, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Workforce(u32);

impl Workforce {
  #[inline]
  pub const fn new(value: u32) -> Self {
    Self(value)
  }
}

impl From<BuildingLevel> for Workforce {
  fn from(value: BuildingLevel) -> Self {
    Workforce(u32::from(*value))
  }
}

impl From<Workforce> for f64 {
  fn from(value: Workforce) -> Self {
    f64::from(value.0)
  }
}

impl From<f64> for Workforce {
  fn from(value: f64) -> Self {
    Self::new(value as u32)
  }
}

impl PartialEq<u32> for Workforce {
  fn eq(&self, other: &u32) -> bool {
    self.0 == *other
  }
}

impl PartialOrd<u32> for Workforce {
  fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
    self.0.partial_cmp(other)
  }
}

impl Add for Workforce {
  type Output = Workforce;

  fn add(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_add(rhs.0))
  }
}

impl AddAssign for Workforce {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl Sub for Workforce {
  type Output = Workforce;

  fn sub(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_sub(rhs.0))
  }
}

impl SubAssign for Workforce {
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}

impl Mul for Workforce {
  type Output = Workforce;

  fn mul(self, rhs: Workforce) -> Self::Output {
    Self(self.0.saturating_mul(rhs.0))
  }
}

impl MulAssign for Workforce {
  fn mul_assign(&mut self, rhs: Self) {
    *self = *self * rhs;
  }
}

impl Mul<f64> for Workforce {
  type Output = f64;

  fn mul(self, rhs: f64) -> Self::Output {
    f64::from(self.0) * rhs
  }
}

impl Mul<Workforce> for f64 {
  type Output = f64;

  fn mul(self, rhs: Workforce) -> Self::Output {
    self * f64::from(rhs.0)
  }
}

impl Mul<Stability> for Workforce {
  type Output = Workforce;

  fn mul(self, rhs: Stability) -> Self::Output {
    Self::from(self.mul_ceil(*rhs))
  }
}

impl MulAssign<Stability> for Workforce {
  fn mul_assign(&mut self, rhs: Stability) {
    *self = *self * rhs;
  }
}

impl_mul_ceil!(Workforce);

#[derive(Clone, Copy, Debug, Deref, Deserialize, Serialize)]
pub struct WorkforceGrowth(f64);

impl WorkforceGrowth {
  #[inline]
  pub const fn new(value: f64) -> Self {
    Self(value.clamp(0.0, 1.0))
  }
}
