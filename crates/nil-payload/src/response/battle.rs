// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::battle::BattleResult;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[cfg(feature = "axum")]
use nil_payload_macros::IntoJsonResponse;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
pub struct SimulateBattleResponse(pub BattleResult);
