// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::{Deref, DerefMut, Display, From, Into};
use nil_core::ethic::Ethics;
use nil_core::npc::bot::BotId;
use serde::{Deserialize, Serialize};

#[cfg(feature = "axum")]
use nil_payload_macros::IntoJsonResponse;

#[cfg(feature = "typescript")]
use ts_rs::TS;

#[derive(Clone, Copy, Debug, Deref, DerefMut, From, Into, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetEthicsResponse(pub Option<Ethics>);

#[derive(Clone, Debug, Deref, DerefMut, Display, From, Into, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatSpawnBotResponse(pub BotId);
