// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Resources } from '@/types/core/resources';

export interface PublicPlayer {
  readonly id: PlayerId;
  readonly status: PlayerStatus;
}

export interface Player extends PublicPlayer {
  readonly resources: Resources;
}

export type PlayerId = string;

export type PlayerStatus = 'active' | 'inactive';

export interface PlayerOptions {
  readonly id: PlayerId;
}
