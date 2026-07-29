// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use bon::Builder;
use nil_core::world::config::WorldId;
use nil_crypto::password::Password;
use serde::Deserialize;

#[derive(Builder, Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
pub struct WebsocketQuery {
  #[builder(start_fn)]
  pub world_id: WorldId,
  #[serde(default)]
  #[builder(into)]
  pub world_password: Option<Password>,
}
