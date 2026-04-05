// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { WorldId } from '@/types/core/world';
import type { PlayerId } from '@/types/core/player';

export interface CheatGetPlayerRequest {
  readonly world: WorldId;
  readonly player?: Option<PlayerId>;
}

export interface CheatGetPlayersRequest {
  readonly world: WorldId;
}
