// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::Coord;
use crate::error::{Error, Result};
use crate::infrastructure::queue::{InfrastructureQueue, InfrastructureQueueOrder};
use crate::military::squad::{Squad, SquadSize};
use crate::military::unit::{AcademyUnitId, UnitBox};
use crate::resources::{Resources, Workforce};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::num::NonZeroU32;
use strum::EnumIs;
use uuid::Uuid;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AcademyRecruitQueue {
  orders: VecDeque<AcademyRecruitOrder>,
}

impl AcademyRecruitQueue {
  pub(crate) fn recruit(
    &mut self,
    request: &AcademyRecruitOrderRequest,
    current_resources: Option<&Resources>,
  ) -> Result<&AcademyRecruitOrder> {
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

    self.orders.push_back(AcademyRecruitOrder {
      id: AcademyRecruitOrderId::new(),
      squad: Squad::new(unit.id(), size),
      resources,
      workforce,
      state: AcademyRecruitOrderState::new(workforce),
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
  pub(crate) fn cancel(&mut self, id: AcademyRecruitOrderId) -> Option<AcademyRecruitOrder> {
    let position = self
      .orders
      .iter()
      .position(|order| order.id == id)?;

    self.orders.remove(position)
  }

  pub fn iter(&self) -> impl Iterator<Item = &AcademyRecruitOrder> {
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

impl InfrastructureQueue<AcademyRecruitOrder> for AcademyRecruitQueue {
  fn queue_mut(&mut self) -> &mut VecDeque<AcademyRecruitOrder> {
    &mut self.orders
  }
}

#[must_use]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AcademyRecruitOrder {
  id: AcademyRecruitOrderId,
  squad: Squad,
  resources: Resources,
  workforce: Workforce,
  state: AcademyRecruitOrderState,
}

impl AcademyRecruitOrder {
  #[inline]
  pub fn id(&self) -> AcademyRecruitOrderId {
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

impl From<AcademyRecruitOrder> for Squad {
  fn from(order: AcademyRecruitOrder) -> Self {
    order.squad
  }
}

impl InfrastructureQueueOrder for AcademyRecruitOrder {
  fn is_done(&self) -> bool {
    self.state.is_done()
  }

  fn set_done(&mut self) {
    self.state = AcademyRecruitOrderState::Done;
  }

  fn pending_workforce_mut(&mut self) -> Option<&mut Workforce> {
    self.state.pending_workforce_mut()
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct AcademyRecruitOrderId(Uuid);

impl AcademyRecruitOrderId {
  #[must_use]
  pub fn new() -> Self {
    Self(Uuid::new_v4())
  }
}

impl Default for AcademyRecruitOrderId {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Clone, Debug, EnumIs, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum AcademyRecruitOrderState {
  Pending { workforce: Workforce },
  Done,
}

impl AcademyRecruitOrderState {
  fn pending_workforce_mut(&mut self) -> Option<&mut Workforce> {
    if let Self::Pending { workforce } = self { Some(workforce) } else { None }
  }
}

impl AcademyRecruitOrderState {
  fn new(workforce: Workforce) -> Self {
    Self::Pending { workforce }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AcademyRecruitOrderRequest {
  pub coord: Coord,
  pub unit: AcademyUnitId,
  pub chunks: NonZeroU32,
}
