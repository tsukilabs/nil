// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod prelude;
mod score;

use crate::city::CityOwner;
use crate::impl_from_ruler;
use crate::military::army::ArmyOwner;
use crate::npc::bot::BotId;
use crate::npc::precursor::PrecursorId;
use crate::player::PlayerId;
use bon::Builder;
use derive_more::Deref;
use itertools::Itertools;
use nil_core_macros::Ruler;
use serde::{Deserialize, Serialize};

pub use score::Score;

#[derive(Clone, Debug, Default, Deref, Deserialize, Serialize)]
pub struct Ranking(Vec<RankingEntry>);

impl Ranking {
  #[inline]
  pub fn get(&self, ruler: &RankingEntryRuler) -> Option<&RankingEntry> {
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
  ruler: RankingEntryRuler,

  #[builder(into)]
  score: Score,

  #[builder(into)]
  cities: u32,
}

#[allow(variant_size_differences)]
#[derive(Ruler, Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum RankingEntryRuler {
  Bot { id: BotId },
  Player { id: PlayerId },
  Precursor { id: PrecursorId },
}

impl_from_ruler!(ArmyOwner => RankingEntryRuler);
impl_from_ruler!(CityOwner => RankingEntryRuler);

impl_from_ruler!(RankingEntryRuler => ArmyOwner);
impl_from_ruler!(RankingEntryRuler => CityOwner);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Rank(u32);
