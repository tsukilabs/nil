// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::World;
use crate::error::{Error, Result};
use crate::ranking::prelude::*;

impl World {
  pub fn get_score<R>(&self, ruler: R) -> Result<Score>
  where
    R: Into<RankingEntryRuler>,
  {
    let ruler: RankingEntryRuler = ruler.into();
    let stats = self.stats.infrastructure.as_ref();
    let mut score = self.military.score_of(ruler.clone());
    score += self
      .continent
      .cities_by(|city| &ruler == city.owner())
      .try_fold(Score::default(), |mut score, city| {
        score += city.score(stats)?;
        Ok::<_, Error>(score)
      })?;

    Ok(score)
  }

  pub(super) fn update_ranking(&mut self) -> Result<()> {
    let len = self.ranking.len();
    let mut entries = Vec::with_capacity(len);

    macro_rules! push {
      ($ruler:expr) => {{
        let ruler = RankingEntryRuler::from($ruler.id());
        let score = self.get_score(ruler.clone())?;
        let cities = self.count_cities(ruler.clone());
        let entry = RankingEntry::builder()
          .ruler(ruler)
          .score(score)
          .cities(cities)
          .build();

        entries.push(entry);

        Ok::<_, Error>(())
      }};
    }

    self
      .player_manager
      .players()
      .try_for_each(|player| push!(player))?;

    self
      .bot_manager
      .bots()
      .try_for_each(|bot| push!(bot))?;

    self
      .precursor_manager
      .precursors()
      .try_for_each(|precursor| push!(precursor))?;

    self.ranking.update(entries);

    Ok(())
  }
}
