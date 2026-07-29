// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use bon::Builder;
use nil_core::continent::coord::Coord;
use nil_core::infrastructure::building::BuildingId;
use nil_core::world::config::WorldId;
use serde::{Deserialize, Serialize};

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct ToggleBuildingRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
  #[builder(into)]
  pub coord: Coord,
  #[builder(into)]
  pub id: BuildingId,
  pub enabled: bool,
}
