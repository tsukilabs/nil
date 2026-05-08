// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::world::config::WorldId;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU8;

#[cfg(feature = "typescript")]
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatSkipRoundRequest {
  pub world: WorldId,
  pub amount: NonZeroU8,
}
