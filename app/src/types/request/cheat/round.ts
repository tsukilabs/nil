// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { WorldId } from '@/types/core/world';

export interface CheatSkipRoundRequest {
  readonly world: WorldId;
  readonly amount: number;
}
