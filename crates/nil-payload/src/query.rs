// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::world::config::WorldId;
use nil_crypto::password::Password;
use serde::Deserialize;
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields = nullable)]
pub struct WebsocketQuery {
  pub world_id: WorldId,
  #[serde(default)]
  pub world_password: Option<Password>,
}
