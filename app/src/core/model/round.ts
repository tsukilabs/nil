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

  public isWaiting() {
    return this.state.kind === 'waiting';
  }

  public isWaitingPlayer(id: PlayerId) {
    return this.state.kind === 'waiting' && this.state.players.includes(id);
  }

  public isDone() {
    return this.state.kind === 'done';
  }

  public static create(round: Round) {
    return new RoundImpl(round);
  }

  public static async load() {
    const round = await commands.getRound();
    return RoundImpl.create(round);
  }
}
