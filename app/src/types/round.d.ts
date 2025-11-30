// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface Round {
  readonly id: RoundId;
  readonly state: RoundState;
}

type RoundId = number;

type RoundState = RoundStateIdle | RoundStateWaiting | RoundStateDone;

interface RoundStateIdle {
  readonly kind: 'idle';
}

interface RoundStateWaiting {
  readonly kind: 'waiting';
  readonly players: readonly PlayerId[];
}

interface RoundStateDone {
  readonly kind: 'done';
}
