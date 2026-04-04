// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type Ranking = readonly RankingEntry[];

interface RankingEntry {
  readonly rank: number;
  readonly ruler: Ruler;
  readonly score: number;
  readonly cities: number;
}
