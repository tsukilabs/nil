// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_client::{Client, ServerAddr};
use nil_core::player::PlayerId;
use nil_core::world::config::WorldId;
use nil_crypto::password::Password;
use nil_server_types::auth::Token;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use tokio::sync::RwLock;

pub(crate) static CLIENT: LazyLock<RwLock<Client>> = LazyLock::new(RwLock::default);

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
#[cfg_attr(feature = "typescript", ts(rename = "ffi_UpdateClient"))]
pub struct UpdateClient {
  pub server: ServerAddr,
  pub world_id: Option<WorldId>,
  pub world_password: Option<Password>,
  pub player_id: Option<PlayerId>,
  pub player_password: Option<Password>,
  pub authorization_token: Option<Token>,
}
