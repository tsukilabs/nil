// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::world::config::WorldId;
use nil_crypto::password::Password;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebsocketQuery {
  pub world_id: WorldId,
  #[serde(default)]
  pub world_password: Option<Password>,
}
