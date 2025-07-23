// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { ArmyPersonnelImpl } from './army-personnel';

export class ArmyImpl implements Army {
  public readonly id: ArmyId;
  public readonly personnel: ArmyPersonnelImpl;
  public readonly state: ArmyState;
  public readonly owner: ArmyOwner;

  private constructor(army: Army) {
    this.id = army.id;
    this.personnel = ArmyPersonnelImpl.create(army.personnel);
    this.state = army.state;
    this.owner = army.owner;
  }

  public static create(army: Army) {
    return new ArmyImpl(army);
  }
}
