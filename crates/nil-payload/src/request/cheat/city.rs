// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use bon::Builder;
use nil_core::city::stability::Stability;
use nil_core::continent::Coord;
use nil_core::world::config::WorldId;
use serde::{Deserialize, Serialize};

#[cfg(feature = "typescript")]
use ts_rs::TS;

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatSetStabilityRequest {
  pub world: WorldId,
  #[builder(into)]
  pub coord: Coord,
  #[builder(into)]
  pub stability: Stability,
}
