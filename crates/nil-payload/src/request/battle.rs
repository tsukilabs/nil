// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use bon::Builder;
use nil_core::battle::luck::Luck;
use nil_core::infrastructure::building::level::BuildingLevel;
use nil_core::military::squad::Squad;
use nil_core::world::config::WorldId;
use serde::{Deserialize, Serialize};

#[cfg(feature = "typescript")]
use ts_rs::TS;

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct SimulateBattleRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
  #[serde(default)]
  #[builder(default, with = FromIterator::from_iter)]
  pub attacker: Vec<Squad>,
  #[serde(default)]
  #[builder(default, with = FromIterator::from_iter)]
  pub defender: Vec<Squad>,
  #[serde(default)]
  #[builder(into)]
  pub luck: Option<Luck>,
  #[serde(default)]
  #[builder(default, into)]
  pub wall: BuildingLevel,
}
