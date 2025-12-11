// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { asyncRef } from '@tb-dev/vue';
import { RankingImpl } from '@/core/model/ranking/ranking';

export function useRanking() {
  const ranking = asyncRef(null, () => RankingImpl.load());

  return {
    ranking: ranking.state,
    loading: ranking.isLoading,
    load: ranking.execute,
  };
}
