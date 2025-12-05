// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';

export class RoundImpl implements Round {
  public readonly id: number;
  public readonly state: RoundState;

  private constructor(round: Round) {
    this.id = round.id;
    this.state = round.state;
  }

  public isIdle() {
    return this.state.kind === 'idle';
  }

  public isDone() {
    return this.state.kind === 'done';
  }

  public isWaiting() {
    return this.state.kind === 'waiting';
  }

  public isWaitingPlayer(id: PlayerId) {
    return this.isPlayerPending(id) || this.isPlayerReady(id);
  }

  public isPlayerPending(id: PlayerId) {
    return this.state.kind === 'waiting' && this.state.pending.includes(id);
  }

  public isPlayerReady(id: PlayerId) {
    return this.state.kind === 'waiting' && this.state.ready.includes(id);
  }

  public static create(round: Round) {
    if (round instanceof RoundImpl) {
      return round;
    }

    return new RoundImpl(round);
  }

  public static async load() {
    const round = await commands.getRound();
    return RoundImpl.create(round);
  }
}
