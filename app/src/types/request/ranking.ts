// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Ruler } from '@/types/core/ruler';
import type { WorldId } from '@/types/core/world';

export interface GetRankingRequest {
  readonly world: WorldId;
}

export interface GetRankRequest {
  readonly world: WorldId;
  readonly ruler: Ruler;
}
