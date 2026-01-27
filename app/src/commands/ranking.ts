// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import type { GetRankingRequest, GetRankRequest } from '@/lib/request';

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
  const req: GetRankRequest = {
    world: NIL.world.getIdStrict(),
    ruler,
  };

  return invoke<Option<RankingEntry>>('get_rank', { req });
}

export async function getRanking() {
  const req: GetRankingRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<Ranking>('get_ranking', { req });
}
