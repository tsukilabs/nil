// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::military::maneuver::ManeuverRequest;
use nil_core::world::WorldId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestManeuverRequest {
  pub world: WorldId,
  pub request: ManeuverRequest,
}
