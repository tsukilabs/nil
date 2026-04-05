// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { ServerAddr } from '@/types/server';
import type { WorldId } from '@/types/core/world';
import type { PlayerId } from '@/types/core/player';

export interface ClientOptions {
  serverAddr: ServerAddr;
  worldId?: Option<WorldId>;
  worldPassword?: Option<string>;
  playerId?: Option<PlayerId>;
  playerPassword?: Option<string>;
  authorizationToken?: Option<string>;
}
