// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::Coord;
use crate::error::{Error, Result};
use crate::infrastructure::building::{BuildingId, BuildingLevel, BuildingStatsTable};
use crate::infrastructure::queue::{InfrastructureQueue, InfrastructureQueueOrder};
use crate::resources::{Resources, Workforce};
use nil_num::ops::MulCeil;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use strum::EnumIs;
use uuid::Uuid;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrefectureBuildQueue {
  orders: VecDeque<PrefectureBuildOrder>,
}

impl PrefectureBuildQueue {
  pub(crate) fn build(
    &mut self,
    request: &PrefectureBuildOrderRequest,
    table: &BuildingStatsTable,
    current_level: BuildingLevel,
    current_resources: Option<&Resources>,
  ) -> Result<&PrefectureBuildOrder> {
    let id = table.id();
    let mut target_level = self.resolve_level(id, current_level);

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

    self.orders.push_back(PrefectureBuildOrder {
      id: PrefectureBuildOrderId::new(),
      kind,
      building: id,
      level: target_level,
      resources,
      workforce,
      state: PrefectureBuildOrderState::new(workforce),
    });

    let len = self.orders.len();
    Ok(unsafe {
      self
        .orders
        .get(len.unchecked_sub(1))
        .unwrap_unchecked()
    })
  }

  /// Cancels the last build order in the queue.
  #[must_use]
  pub(crate) fn cancel(&mut self) -> Option<PrefectureBuildOrder> {
    self.orders.pop_back()
  }

  pub fn iter(&self) -> impl Iterator<Item = &PrefectureBuildOrder> {
    self.orders.iter()
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.orders.len()
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.orders.is_empty()
  }

  pub fn resolve_level(&self, building: BuildingId, current_level: BuildingLevel) -> BuildingLevel {
    self
      .iter()
      .filter(|order| order.building() == building)
      .fold(current_level, |acc, order| {
        match order.kind() {
          PrefectureBuildOrderKind::Construction => acc + 1u8,
          PrefectureBuildOrderKind::Demolition => acc - 1u8,
        }
      })
  }

  pub fn sum_workforce(&self) -> Workforce {
    self
      .iter()
      .map(|order| u32::from(order.workforce))
      .sum::<u32>()
      .into()
  }
}

impl InfrastructureQueue<PrefectureBuildOrder> for PrefectureBuildQueue {
  fn queue_mut(&mut self) -> &mut VecDeque<PrefectureBuildOrder> {
    &mut self.orders
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
  state: PrefectureBuildOrderState,
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
}

impl InfrastructureQueueOrder for PrefectureBuildOrder {
  fn is_done(&self) -> bool {
    self.state.is_done()
  }

  fn set_done(&mut self) {
    self.state = PrefectureBuildOrderState::Done;
  }

  fn pending_mut(&mut self) -> Option<&mut Workforce> {
    self.state.workforce_mut()
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct PrefectureBuildOrderId(Uuid);

impl PrefectureBuildOrderId {
  #[must_use]
  pub fn new() -> Self {
    Self(Uuid::new_v4())
  }
}

impl Default for PrefectureBuildOrderId {
  fn default() -> Self {
    Self::new()
  }
}

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

#[derive(Clone, Debug, EnumIs, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum PrefectureBuildOrderState {
  Pending { workforce: Workforce },
  Done,
}

impl PrefectureBuildOrderState {
  fn workforce_mut(&mut self) -> Option<&mut Workforce> {
    if let Self::Pending { workforce } = self { Some(workforce) } else { None }
  }
}

impl PrefectureBuildOrderState {
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
