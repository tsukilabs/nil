// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod army;
pub mod squad;
pub mod unit;

use crate::continent::{ContinentIndex, ContinentSize, IntoContinentIndex};
use crate::military::army::Army;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Military {
  grid: HashMap<ContinentIndex, Vec<Army>>,
  continent_size: ContinentSize,
}

// TODO: All armies should be reconciled when the round ends to avoid having
// multiple copies of idle armies owned by the same entity in the same location.
impl Military {
  pub(crate) fn new(size: ContinentSize) -> Self {
    Self {
      grid: HashMap::new(),
      continent_size: size,
    }
  }

  pub fn armies_at<I>(&self, index: I) -> &[Army]
  where
    I: IntoContinentIndex,
  {
    let index = index.into_index(self.continent_size);
    self
      .grid
      .get(&index)
      .map(Vec::as_slice)
      .unwrap_or_default()
  }

  pub fn idle_armies_at<I>(&self, index: I) -> impl Iterator<Item = &Army>
  where
    I: IntoContinentIndex,
  {
    self
      .armies_at(index)
      .iter()
      .filter(|army| army.is_idle())
  }

  pub(crate) fn insert<I>(&mut self, index: I, army: Army)
  where
    I: IntoContinentIndex,
  {
    let index = index.into_index(self.continent_size);
    self
      .grid
      .entry(index)
      .or_default()
      .push(army);
  }
}
