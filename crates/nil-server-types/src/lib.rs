// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]
#![feature(str_as_str)]

use jiff::Zoned;
use nil_core::continent::ContinentSize;
use nil_core::player::PlayerId;
use nil_core::round::RoundId;
use nil_core::world::config::{WorldConfig, WorldId};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Token(Box<str>);

impl Token {
  pub fn new(token: impl AsRef<str>) -> Self {
    Self(Box::from(token.as_ref()))
  }
}

impl AsRef<str> for Token {
  fn as_ref(&self) -> &str {
    self.0.as_str()
  }
}

impl AsRef<[u8]> for Token {
  fn as_ref(&self) -> &[u8] {
    self.0.as_bytes()
  }
}

impl Deref for Token {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    self.0.as_str()
  }
}
