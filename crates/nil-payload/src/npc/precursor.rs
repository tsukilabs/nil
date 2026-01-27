// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::npc::precursor::PrecursorId;
use nil_core::world::WorldId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPrecursorCoordsRequest {
  pub world: WorldId,
  pub id: PrecursorId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPublicPrecursorRequest {
  pub world: WorldId,
  pub id: PrecursorId,
}
