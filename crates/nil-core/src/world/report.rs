// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::player::PlayerId;
use crate::report::ReportId;
use crate::world::World;

impl World {
  pub fn forward_report(&mut self, id: ReportId, recipient: PlayerId) {
    if self.report.forward(id, recipient.clone()) {
      self.emit_report(recipient, id);
    }
  }
}
