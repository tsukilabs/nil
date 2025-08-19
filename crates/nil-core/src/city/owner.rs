// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::npc::bot::BotId;
use crate::npc::precursor::PrecursorId;
use crate::player::PlayerId;
use nil_core_macros::Ruler;
use serde::{Deserialize, Serialize};

#[allow(variant_size_differences)]
#[derive(Ruler, Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum CityOwner {
  Bot { id: BotId },
  Player { id: PlayerId },
  Precursor { id: PrecursorId },
}
