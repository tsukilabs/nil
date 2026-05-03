// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::battle::BattleResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(nil_payload_macros::IntoJsonResponse))]
pub struct SimulateBattleResponse(pub BattleResult);
