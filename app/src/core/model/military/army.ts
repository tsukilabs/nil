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
    // eslint-disable-next-line @typescript-eslint/no-unnecessary-condition
    return this.state === 'idle';
  }

  public static create(army: Army) {
    return new ArmyImpl(army);
  }
}
