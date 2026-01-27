// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::world::World;
use std::fmt;
use std::sync::Arc;

#[derive(Clone)]
pub struct OnNextRound(Arc<dyn Fn(&mut World) + Send + Sync>);

impl OnNextRound {
  pub fn new<F>(f: F) -> Self
  where
    F: Fn(&mut World) + Send + Sync + 'static,
  {
    Self(Arc::new(f))
  }

  pub(crate) fn call(&self, world: &mut World) {
    (self.0)(world);
  }
}

impl fmt::Debug for OnNextRound {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("OnNextRound")
      .finish_non_exhaustive()
  }
}
