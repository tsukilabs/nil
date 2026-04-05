// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { PlayerId } from '@/types/core/player';

export type Ruler = RulerBot | RulerPlayer | RulerPrecursor;

export type RulerId = Ruler['id'];
export type RulerKind = Ruler['kind'];

export interface RulerBot {
  readonly kind: 'bot';
  readonly id: BotId;
}

export interface RulerPlayer {
  readonly kind: 'player';
  readonly id: PlayerId;
}

export interface RulerPrecursor {
  readonly kind: 'precursor';
  readonly id: PrecursorId;
}
