// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::diplomacy::opinion::Opinion;
use crate::ruler::Ruler;
use crate::world::World;

impl World {
  pub fn get_opinion(&self, left: &Ruler, right: &Ruler) -> Option<Opinion> {
    if left.is_player() || left == right {
      return None;
    } else if left.is_precursor() && right.is_precursor() {
      return Some(Opinion::MIN);
    }

    Some(Opinion::default())
  }
}
