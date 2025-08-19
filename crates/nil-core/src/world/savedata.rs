// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::event::Emitter;
use crate::savedata::Savedata;
use crate::world::World;
use jiff::Zoned;

impl World {
  pub(super) fn consume_pending_save(&mut self) -> Result<()> {
    if let Some(mut path) = self.pending_save.take() {
      let mut data = Savedata::from(&*self);
      path.set_extension("nil");
      data.save(&path)?;
    }

    Ok(())
  }
}

impl From<&World> for Savedata {
  fn from(world: &World) -> Self {
    Self {
      round: world.round.to_idle(),
      continent: world.continent.clone(),
      player_manager: world.player_manager.clone(),
      bot_manager: world.bot_manager.clone(),
      precursor_manager: world.precursor_manager.clone(),
      military: world.military.clone(),
      ranking: world.ranking.clone(),
      config: world.config.clone(),
      stats: world.stats.clone(),
      chat: world.chat.clone(),
      scripting: world.scripting.clone(),

      saved_at: Zoned::now(),
    }
  }
}

impl From<Savedata> for World {
  fn from(savedata: Savedata) -> Self {
    Self {
      round: savedata.round,
      continent: savedata.continent,
      player_manager: savedata.player_manager,
      bot_manager: savedata.bot_manager,
      precursor_manager: savedata.precursor_manager,
      military: savedata.military,
      ranking: savedata.ranking,
      config: savedata.config,
      stats: savedata.stats,
      chat: savedata.chat,
      scripting: savedata.scripting,

      emitter: Emitter::default(),
      pending_save: None,
    }
  }
}
