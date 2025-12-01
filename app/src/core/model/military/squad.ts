// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export class SquadImpl implements Squad {
  public readonly unit: UnitId;
  public readonly size: number;

  private constructor(squad: Squad) {
    this.unit = squad.unit;
    this.size = squad.size;
  }

  public isEmpty() {
    return this.size === 0;
  }

  public static create(squad: Squad) {
    if (squad instanceof SquadImpl) {
      return squad;
    }

    return new SquadImpl(squad);
  }
}
