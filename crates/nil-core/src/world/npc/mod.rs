// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod bot;
mod precursor;

use crate::error::Result;
use crate::ethic::Ethics;
use crate::ruler::Ruler;
use crate::world::World;
use tap::Pipe;

impl World {
  pub(crate) fn get_ethics(&self, ruler: &Ruler) -> Result<Option<Ethics>> {
    self.ruler(ruler)?.ethics().copied().pipe(Ok)
  }
}
