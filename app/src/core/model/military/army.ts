// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { ArmyPersonnelImpl } from './army-personnel';

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
