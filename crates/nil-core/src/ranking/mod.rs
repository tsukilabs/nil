// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod prelude;
mod score;

use crate::ruler::Ruler;
use bon::Builder;
use derive_more::Deref;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

pub use score::Score;

#[derive(Clone, Debug, Default, Deref, Deserialize, Serialize)]
pub struct Ranking(Vec<RankingEntry>);

impl Ranking {
  #[inline]
  pub fn get(&self, ruler: &Ruler) -> Option<&RankingEntry> {
    self
      .0
      .iter()
      .find(|entry| &entry.ruler == ruler)
  }

  pub fn update<T>(&mut self, entries: T)
  where
    T: IntoIterator<Item = RankingEntry>,
  {
    self.0.clear();
    let entries = entries
      .into_iter()
      .sorted_by_key(|it| it.score)
      .rev()
      .zip(1u32..)
      .map(|(mut entry, rank)| {
        entry.rank = Rank(rank);
        entry
      });

    self.0.extend(entries);
  }
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RankingEntry {
  #[builder(skip)]
  rank: Rank,

  #[builder(into)]
  ruler: Ruler,

  #[builder(into)]
  score: Score,

  #[builder(into)]
  cities: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Rank(u32);
