// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::infrastructure::building::{BuildingId, BuildingLevel, BuildingStatsTable};
use crate::resource::{Resources, Workforce};
use crate::village::Coord;
use derive_more::Deref;
use nil_num::BigIntU64;
use nil_num::ops::MulCeil;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use strum::EnumIs;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrefectureBuildQueue {
  orders: VecDeque<PrefectureBuildOrder>,
  current_id: PrefectureBuildOrderId,
}

impl PrefectureBuildQueue {
  pub(crate) fn build(
    &mut self,
    table: &BuildingStatsTable,
    current_level: BuildingLevel,
    current_resources: Option<&Resources>,
    request: &PrefectureBuildOrderRequest,
  ) -> Result<&PrefectureBuildOrder> {
    let id = table.id();
    let mut target_level = self
      .iter()
      .filter(|order| order.building() == id)
      .fold(current_level, |acc, order| {
        match order.kind() {
          PrefectureBuildOrderKind::Construction => acc + 1u8,
          PrefectureBuildOrderKind::Demolition => acc - 1u8,
        }
      });

    let kind = request.kind;
    if kind.is_demolition() && target_level <= table.min_level() {
      return Err(Error::CannotDecreaseBuildingLevel(id));
    } else if kind.is_construction() && target_level >= table.max_level() {
      return Err(Error::CannotIncreaseBuildingLevel(id));
    }

    target_level += match kind {
      PrefectureBuildOrderKind::Construction => 1i8,
      PrefectureBuildOrderKind::Demolition => -1i8,
    };

    let resources = table.get(target_level)?.resources.clone();
    if let PrefectureBuildOrderKind::Construction = kind
      && let Some(current_resources) = current_resources
      && current_resources
        .checked_sub(&resources)
        .is_none()
    {
      return Err(Error::InsufficientResources);
    }

    let mut workforce = table.get(target_level)?.workforce;
    kind.apply_modifier(&mut workforce);

    self.current_id = self.current_id.next();
    self.orders.push_back(PrefectureBuildOrder {
      id: self.current_id,
      kind,
      building: id,
      level: target_level,
      resources,
      workforce,
      status: PrefectureBuildOrderStatus::new(workforce),
    });

    let len = self.orders.len();
    Ok(unsafe {
      self
        .orders
        .get(len.unchecked_sub(1))
        .unwrap_unchecked()
    })
  }

  /// Cancela a última ordem de construção da fila.
  #[must_use]
  pub(crate) fn cancel(&mut self) -> Option<PrefectureBuildOrder> {
    self.orders.pop_back()
  }

  /// Consome força de trabalho até que ela se esgote ou toda a fila de construção seja concluída.
  #[must_use]
  pub(super) fn process(&mut self, mut workforce: Workforce) -> Vec<PrefectureBuildOrder> {
    let mut orders = Vec::new();
    loop {
      if workforce == 0 {
        break;
      }

      match self
        .orders
        .pop_front_if(|order| order.update(&mut workforce))
      {
        Some(order) => orders.push(order),
        None => break,
      }
    }

    if !orders.is_empty() {
      self.orders.shrink_to_fit();
    }

    orders
  }

  pub fn iter(&self) -> impl Iterator<Item = &PrefectureBuildOrder> {
    self.orders.iter()
  }
}

#[must_use]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrefectureBuildOrder {
  id: PrefectureBuildOrderId,
  kind: PrefectureBuildOrderKind,
  building: BuildingId,
  level: BuildingLevel,
  resources: Resources,
  workforce: Workforce,
  status: PrefectureBuildOrderStatus,
}

impl PrefectureBuildOrder {
  #[inline]
  pub fn id(&self) -> PrefectureBuildOrderId {
    self.id
  }

  #[inline]
  pub fn kind(&self) -> PrefectureBuildOrderKind {
    self.kind
  }

  #[inline]
  pub fn building(&self) -> BuildingId {
    self.building
  }

  #[inline]
  pub fn resources(&self) -> &Resources {
    &self.resources
  }

  /// Consome força de trabalho para avançar o progresso da ordem de construção.
  ///
  /// Retorna um booleano indicando se a construção foi concluída.
  fn update(&mut self, workforce: &mut Workforce) -> bool {
    if let Some(pending) = self.status.pending() {
      if *pending > 0 {
        let previous = *pending;
        *pending -= *workforce;

        // Reduz a força de trabalho disponível com base na
        // quantidade usada para essa ordem de construção.
        *workforce -= previous - *pending;
      }

      if *pending == 0 {
        self.status = PrefectureBuildOrderStatus::Done;
      }
    }

    self.status.is_done()
  }
}

#[derive(Clone, Copy, Debug, Default, Deref, PartialEq, Eq, BigIntU64)]
pub struct PrefectureBuildOrderId(u64);

impl PrefectureBuildOrderId {
  #[inline]
  #[must_use]
  pub const fn next(self) -> Self {
    Self(self.0.wrapping_add(1))
  }
}

/// Tipo da ordem de construção.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize, EnumIs)]
#[serde(rename_all = "kebab-case")]
pub enum PrefectureBuildOrderKind {
  Construction,
  Demolition,
}

impl PrefectureBuildOrderKind {
  fn apply_modifier(self, workforce: &mut Workforce) {
    if let Self::Demolition = self {
      *workforce = workforce.mul_ceil(0.5).into();
    }
  }
}

/// Indica o progresso da construção.
#[derive(Clone, Debug, EnumIs, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum PrefectureBuildOrderStatus {
  Done,
  Pending { workforce: Workforce },
}

impl PrefectureBuildOrderStatus {
  fn pending(&mut self) -> Option<&mut Workforce> {
    if let Self::Pending { workforce } = self {
      Some(workforce)
    } else {
      None
    }
  }
}

impl PrefectureBuildOrderStatus {
  fn new(workforce: Workforce) -> Self {
    Self::Pending { workforce }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrefectureBuildOrderRequest {
  pub coord: Coord,
  pub building: BuildingId,
  pub kind: PrefectureBuildOrderKind,
}
