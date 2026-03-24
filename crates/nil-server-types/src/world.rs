// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::round::RoundDuration;
use jiff::Zoned;
use nil_core::continent::ContinentSize;
use nil_core::player::PlayerId;
use nil_core::round::RoundId;
use nil_core::world::config::WorldConfig;
use serde::{Deserialize, Serialize};

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
  pub round_duration: Option<RoundDuration>,
  pub active_players: usize,
  pub total_players: usize,
  pub continent_size: ContinentSize,
}
