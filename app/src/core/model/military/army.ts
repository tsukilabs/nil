// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Ruler } from '@/types/core/ruler';
import { ArmyPersonnelImpl } from './army-personnel';
import type { Army, ArmyId, ArmyState } from '@/types/core/military/army';

export class ArmyImpl implements Army {
  public readonly id: ArmyId;
  public readonly personnel: ArmyPersonnelImpl;
  public readonly state: ArmyState;
  public readonly owner: Ruler;

  private constructor(army: Army) {
    this.id = army.id;
    this.personnel = ArmyPersonnelImpl.create(army.personnel);
    this.state = army.state;
    this.owner = army.owner;
  }

  public *[Symbol.iterator]() {
    yield* this.personnel;
  }

  public getSquads() {
    return this.personnel.getSquads();
  }

  public isIdle() {
    return this.state.kind === 'idle';
  }

  public isManeuvering() {
    return this.state.kind === 'maneuvering';
  }

  public static create(army: Army) {
    if (army instanceof ArmyImpl) {
      return army;
    }

    return new ArmyImpl(army);
  }
}
