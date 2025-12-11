// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::Coord;
use crate::error::{Error, Result};
use crate::infrastructure::queue::{InfrastructureQueue, InfrastructureQueueOrder};
use crate::military::squad::{Squad, SquadSize};
use crate::military::unit::{StableUnitId, UnitBox};
use crate::resources::{Resources, Workforce};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::num::NonZeroU32;
use strum::EnumIs;
use uuid::Uuid;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StableRecruitQueue {
  orders: VecDeque<StableRecruitOrder>,
}

impl StableRecruitQueue {
  pub(crate) fn recruit(
    &mut self,
    request: &StableRecruitOrderRequest,
    current_resources: Option<&Resources>,
  ) -> Result<&StableRecruitOrder> {
    let unit = UnitBox::from(request.unit);
    let chunk = unit.as_dyn().chunk();
    let size = SquadSize::new(chunk.size() * request.chunks);
    let resources = &chunk.resources() * request.chunks;
    let workforce = chunk.workforce() * request.chunks;

    if let Some(current_resources) = current_resources
      && current_resources
        .checked_sub(&resources)
        .is_none()
    {
      return Err(Error::InsufficientResources);
    }

    self.orders.push_back(StableRecruitOrder {
      id: StableRecruitOrderId::new(),
      squad: Squad::new(unit.id(), size),
      resources,
      workforce,
      state: StableRecruitOrderState::new(workforce),
    });

    let len = self.orders.len();
    Ok(unsafe {
      self
        .orders
        .get(len.unchecked_sub(1))
        .unwrap_unchecked()
    })
  }

  /// Cancels a recruit order.
  #[must_use]
  pub(crate) fn cancel(&mut self, id: StableRecruitOrderId) -> Option<StableRecruitOrder> {
    let position = self
      .orders
      .iter()
      .position(|order| order.id == id)?;

    self.orders.remove(position)
  }

  pub fn iter(&self) -> impl Iterator<Item = &StableRecruitOrder> {
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
}

impl InfrastructureQueue<StableRecruitOrder> for StableRecruitQueue {
  fn queue_mut(&mut self) -> &mut VecDeque<StableRecruitOrder> {
    &mut self.orders
  }
}

#[must_use]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StableRecruitOrder {
  id: StableRecruitOrderId,
  squad: Squad,
  resources: Resources,
  workforce: Workforce,
  state: StableRecruitOrderState,
}

impl StableRecruitOrder {
  #[inline]
  pub fn id(&self) -> StableRecruitOrderId {
    self.id
  }

  #[inline]
  pub fn squad(&self) -> &Squad {
    &self.squad
  }

  #[inline]
  pub fn resources(&self) -> &Resources {
    &self.resources
  }
}

impl From<StableRecruitOrder> for Squad {
  fn from(order: StableRecruitOrder) -> Self {
    order.squad
  }
}

impl InfrastructureQueueOrder for StableRecruitOrder {
  fn is_done(&self) -> bool {
    self.state.is_done()
  }

  fn set_done(&mut self) {
    self.state = StableRecruitOrderState::Done;
  }

  fn pending_workforce_mut(&mut self) -> Option<&mut Workforce> {
    self.state.pending_workforce_mut()
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct StableRecruitOrderId(Uuid);

impl StableRecruitOrderId {
  #[must_use]
  pub fn new() -> Self {
    Self(Uuid::new_v4())
  }
}

impl Default for StableRecruitOrderId {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Clone, Debug, EnumIs, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum StableRecruitOrderState {
  Pending { workforce: Workforce },
  Done,
}

impl StableRecruitOrderState {
  fn pending_workforce_mut(&mut self) -> Option<&mut Workforce> {
    if let Self::Pending { workforce } = self { Some(workforce) } else { None }
  }
}

impl StableRecruitOrderState {
  fn new(workforce: Workforce) -> Self {
    Self::Pending { workforce }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StableRecruitOrderRequest {
  pub coord: Coord,
  pub unit: StableUnitId,
  pub chunks: NonZeroU32,
}
