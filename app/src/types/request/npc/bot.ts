// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { BotId } from '@/types/core/npc/bot';
import type { WorldId } from '@/types/core/world';

export interface GetBotCoordsRequest {
  readonly world: WorldId;
  readonly id: BotId;
}

export interface GetPublicBotRequest {
  readonly world: WorldId;
  readonly id: BotId;
}

export interface GetPublicBotsRequest {
  readonly world: WorldId;
}
