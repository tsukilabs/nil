// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export function getBotRank(id: BotId) {
  return getRank({ kind: 'bot', id });
}

export function getPlayerRank(id: PlayerId) {
  return getRank({ kind: 'player', id });
}

export function getPrecursorRank(id: PrecursorId) {
  return getRank({ kind: 'precursor', id });
}

export function getRank(id: RankingEntryRuler) {
  return invoke<Option<RankingEntry>>('get_rank', { id });
}

export function getRanking() {
  return invoke<Ranking>('get_ranking');
}
