// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type Ranking = readonly RankingEntry[];

interface RankingEntry {
  readonly rank: number;
  readonly ruler: RankingEntryRuler;
  readonly score: number;
  readonly cities: number;
}

type RankingEntryRuler = RankingEntryIdBot | RankingEntryIdPlayer | RankingEntryIdPrecursor;

type RankingEntryRulerKind = RankingEntryRuler['kind'];

interface RankingEntryIdBot {
  readonly kind: 'bot';
  readonly id: BotId;
}

interface RankingEntryIdPlayer {
  readonly kind: 'player';
  readonly id: PlayerId;
}

interface RankingEntryIdPrecursor {
  readonly kind: 'precursor';
  readonly id: PrecursorId;
}
