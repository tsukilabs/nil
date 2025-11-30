// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export async function getBotRank(id: BotId) {
  return getRank({ kind: 'bot', id });
}

export async function getPlayerRank(id: PlayerId) {
  return getRank({ kind: 'player', id });
}

export async function getPrecursorRank(id: PrecursorId) {
  return getRank({ kind: 'precursor', id });
}

export async function getRank(ruler: Ruler) {
  return invoke<Option<RankingEntry>>('get_rank', { req: { ruler } });
}

export async function getRanking() {
  return invoke<Ranking>('get_ranking');
}
