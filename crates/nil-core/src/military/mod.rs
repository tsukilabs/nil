// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod army;
pub mod squad;
pub mod unit;

use crate::continent::{ContinentIndex, ContinentKey, ContinentSize};
use crate::military::army::{Army, ArmyOwner, ArmyPersonnel};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Military {
  continent: HashMap<ContinentIndex, Vec<Army>>,
  continent_size: ContinentSize,
}

// TODO: All armies should be reconciled when the round ends to avoid having
// multiple copies of idle armies owned by the same entity in the same location.
impl Military {
  pub(crate) fn new(size: ContinentSize) -> Self {
    Self {
      continent: HashMap::new(),
      continent_size: size,
    }
  }

  pub fn armies_at<K>(&self, key: K) -> &[Army]
  where
    K: ContinentKey,
  {
    let index = key.into_index(self.continent_size);
    self
      .continent
      .get(&index)
      .map(Vec::as_slice)
      .unwrap_or_default()
  }

  pub fn idle_armies_at<K>(&self, key: K) -> impl Iterator<Item = &Army>
  where
    K: ContinentKey,
  {
    self
      .armies_at(key)
      .iter()
      .filter(|army| army.is_idle())
  }

  #[must_use]
  pub fn intersection<K, I>(&self, keys: I) -> Self
  where
    K: ContinentKey,
    I: IntoIterator<Item = K>,
  {
    let mut military = Self::new(self.continent_size);
    for key in keys {
      let index = key.into_index(self.continent_size);
      if let Some(armies) = self.continent.get(&index).cloned() {
        military.continent.insert(index, armies);
      }
    }

    military
  }

  pub(crate) fn spawn<K, O>(&mut self, key: K, owner: O, personnel: ArmyPersonnel)
  where
    K: ContinentKey,
    O: Into<ArmyOwner>,
  {
    let index = key.into_index(self.continent_size);
    let army = Army::builder()
      .owner(owner)
      .personnel(personnel)
      .build();

    self
      .continent
      .entry(index)
      .or_default()
      .push(army);
  }
}
