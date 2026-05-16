// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use bon::Builder;
use nil_core::world::config::WorldId;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU8;

#[cfg(feature = "typescript")]
use ts_rs::TS;

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatSkipRoundRequest {
  #[builder(start_fn)]
  pub world: WorldId,
  #[builder(default = NonZeroU8::MIN)]
  pub amount: NonZeroU8,
}
