// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::world::WorldId;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU8;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheatSkipRoundRequest {
  pub world: WorldId,
  pub amount: NonZeroU8,
}
