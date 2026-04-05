// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Ethics } from '@/types/core/ethic';
import type { Resources } from '@/types/core/resources';

export interface PublicBot {
  readonly id: BotId;
}

export interface Bot extends PublicBot {
  readonly ethics: Ethics;
  readonly resources: Resources;
}

export type BotId = string;
