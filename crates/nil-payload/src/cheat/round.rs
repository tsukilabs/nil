// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheatSkipRoundRequest {
  pub amount: NonZeroU32,
}
