// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::chat::Chat;
use crate::continent::Continent;
use crate::error::{Error, Result};
use crate::military::Military;
use crate::npc::bot::BotManager;
use crate::npc::precursor::PrecursorManager;
use crate::player::{PlayerManager, PlayerStatus};
use crate::round::Round;
use crate::script::Scripting;
use crate::world::{WorldConfig, WorldStats};
use jiff::Zoned;
use nil_util::serde::{read_file, write_file};
use serde::{Deserialize, Serialize};
use std::path::Path;

const COMPRESS: bool = cfg!(any(
  not(debug_assertions),
  target_os = "android",
  target_os = "ios"
));

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Savedata {
  pub(crate) round: Round,
  pub(crate) continent: Continent,
  pub(crate) player_manager: PlayerManager,
  pub(crate) bot_manager: BotManager,
  pub(crate) precursor_manager: PrecursorManager,
  pub(crate) military: Military,
  pub(crate) config: WorldConfig,
  pub(crate) stats: WorldStats,
  pub(crate) chat: Chat,
  pub(crate) scripting: Scripting,

  pub(crate) saved_at: Zoned,
}

impl Savedata {
  pub fn load(path: &Path) -> Result<Self> {
    read_file(path)
      .decode(COMPRESS)
      .call()
      .map_err(|_| Error::FailedToLoadWorld)
  }

  pub(crate) fn save(&mut self, path: &Path) -> Result<()> {
    for player in self.player_manager.players_mut() {
      *player.status_mut() = PlayerStatus::Inactive;
    }

    write_file(path, self)
      .encode(COMPRESS)
      .call()
      .map_err(|_| Error::FailedToSaveWorld)
  }
}
