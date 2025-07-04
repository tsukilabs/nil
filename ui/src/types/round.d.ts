// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface Round {
  readonly id: RoundId;
  readonly phase: RoundPhase;
}

type RoundId = number;

type RoundPhase = RoundPhaseIdle | RoundPhasePlayer;

interface RoundPhaseIdle {
  readonly kind: 'idle';
}

interface RoundPhasePlayer {
  readonly kind: 'player';
  readonly pending: readonly PlayerId[];
}
