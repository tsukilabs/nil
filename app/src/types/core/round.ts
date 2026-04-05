// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { PlayerId } from '@/types/core/player';

export interface Round {
  readonly id: RoundId;
  readonly state: RoundState;
  readonly startedAt: string;
}

export type RoundId = number;

export type RoundState = RoundStateIdle | RoundStateWaiting | RoundStateDone;

export interface RoundStateIdle {
  readonly kind: 'idle';
}

export interface RoundStateWaiting {
  readonly kind: 'waiting';
  readonly pending: readonly PlayerId[];
  readonly ready: readonly PlayerId[];
}

export interface RoundStateDone {
  readonly kind: 'done';
}
