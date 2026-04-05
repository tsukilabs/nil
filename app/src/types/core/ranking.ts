// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Ruler } from '@/types/core/ruler';

export type Ranking = readonly RankingEntry[];

export interface RankingEntry {
  readonly rank: number;
  readonly ruler: Ruler;
  readonly score: number;
  readonly cities: number;
}
