// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Ruler } from '@/types/core/ruler';
import type { Ethics } from '@/types/core/ethic';
import type { BotId } from '@/types/core/npc/bot';
import type { WorldId } from '@/types/core/world';
import type { Infrastructure } from '@/types/core/infrastructure';

export interface CheatGetEthicsRequest {
  readonly world: WorldId;
  readonly ruler: Ruler;
}

export interface CheatSetBotEthicsRequest {
  readonly world: WorldId;
  readonly id: BotId;
  readonly ethics: Ethics;
}

export interface CheatSpawnBotRequest {
  readonly world: WorldId;
  readonly name: string;
  readonly infrastructure: Option<Infrastructure>;
}
