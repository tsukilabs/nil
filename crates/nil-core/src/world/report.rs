// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::player::PlayerId;
use crate::report::ReportKind;
use crate::world::World;

impl World {
  /// Forwards a report to a player.
  pub fn forward_report(&self, recipient: PlayerId, report: ReportKind) -> Result<()> {
    if self.has_player(&recipient) {
      self.emit_report(recipient, report)?;
    }

    Ok(())
  }
}
