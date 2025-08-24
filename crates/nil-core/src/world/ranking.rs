// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::World;
use crate::error::{Error, Result};
use crate::ranking::prelude::*;
use crate::ruler::Ruler;

impl World {
  pub fn get_score<R>(&self, ruler: R) -> Result<Score>
  where
    R: Into<Ruler>,
  {
    let ruler: Ruler = ruler.into();
    let stats = self.stats.infrastructure.as_ref();
    let mut score = self.military.score_of(ruler.clone());
    score += self
      .continent
      .cities_of(ruler)
      .try_fold(Score::default(), |mut score, city| {
        score += city.score(stats)?;
        Ok::<_, Error>(score)
      })?;

    Ok(score)
  }

  pub(super) fn update_ranking(&mut self) -> Result<()> {
    let len = self.ranking.len();
    let mut entries = Vec::with_capacity(len);

    for ruler in self.rulers() {
      let ruler = Ruler::from(ruler);
      let score = self.get_score(ruler.clone())?;
      let cities = self.count_cities(ruler.clone());
      let entry = RankingEntry::builder()
        .ruler(ruler)
        .score(score)
        .cities(cities)
        .build();

      entries.push(entry);
    }

    self.ranking.update(entries);

    Ok(())
  }
}
