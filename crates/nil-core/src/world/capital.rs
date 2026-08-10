// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::ruler::Ruler;
use crate::world::World;
use tap::Pipe;

impl World {
  pub fn get_city_limit<R>(&self, ruler: R) -> Result<u32>
  where
    R: Into<Ruler>,
  {
    let ruler: Ruler = ruler.into();
    self
      .ruler(&ruler)?
      .influence()
      .city_limit()
      .pipe(Ok)
  }
}
