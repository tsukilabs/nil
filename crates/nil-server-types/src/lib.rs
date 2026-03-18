// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]
#![feature(str_as_str)]

pub mod auth;
pub mod round;
pub mod time;

use jiff::Zoned;
use nil_core::continent::ContinentSize;
use nil_core::player::PlayerId;
use nil_core::round::RoundId;
use nil_core::world::config::{WorldConfig, WorldId};
use serde::{Deserialize, Serialize};
use strum::EnumIs;

#[derive(Clone, Copy, Debug, EnumIs, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ServerKind {
  Local { id: WorldId },
  Remote,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteWorld {
  pub config: WorldConfig,
  pub description: Option<String>,
  pub created_by: PlayerId,
  pub created_at: Zoned,
  pub updated_at: Zoned,
  pub has_password: bool,
  pub current_round: RoundId,
  pub active_players: usize,
  pub total_players: usize,
  pub continent_size: ContinentSize,
}
