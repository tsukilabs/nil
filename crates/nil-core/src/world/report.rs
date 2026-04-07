// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::player::PlayerId;
use crate::report::{ReportId, ReportManager};
use crate::world::World;

impl World {
  #[inline]
  pub fn report_manager(&self) -> &ReportManager {
    &self.report_manager
  }

  pub fn forward_report(&mut self, id: ReportId, recipient: PlayerId) {
    if self
      .report_manager
      .forward(id, recipient.clone())
    {
      self.emit_report(recipient, id);
    }
  }
}
