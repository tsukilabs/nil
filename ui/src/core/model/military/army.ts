// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { ArmyPersonnelImpl } from './army-personnel';

export class ArmyImpl implements Army {
  public readonly personnel: ArmyPersonnelImpl;
  public readonly state: ArmyState;
  public readonly owner: ArmyOwner;

  private constructor(args: { personnel: ArmyPersonnelImpl; state: ArmyState; owner: ArmyOwner }) {
    this.personnel = args.personnel;
    this.state = args.state;
    this.owner = args.owner;
  }

  public static create(army: Army) {
    return new ArmyImpl({
      personnel: ArmyPersonnelImpl.create(army.personnel),
      state: army.state,
      owner: army.owner,
    });
  }
}
