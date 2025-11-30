// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { SquadImpl } from './squad';

export class ArmyPersonnelImpl implements ArmyPersonnel {
  public readonly archer: SquadImpl;
  public readonly axeman: SquadImpl;
  public readonly heavyCavalry: SquadImpl;
  public readonly lightCavalry: SquadImpl;
  public readonly pikeman: SquadImpl;
  public readonly swordsman: SquadImpl;

  private constructor(personnel: ArmyPersonnel) {
    this.archer = SquadImpl.create(personnel.archer);
    this.axeman = SquadImpl.create(personnel.axeman);
    this.heavyCavalry = SquadImpl.create(personnel.heavyCavalry);
    this.lightCavalry = SquadImpl.create(personnel.lightCavalry);
    this.pikeman = SquadImpl.create(personnel.pikeman);
    this.swordsman = SquadImpl.create(personnel.swordsman);
  }

  public size() {
    return ArmyPersonnelImpl.size(this);
  }

  public isEmpty() {
    return (
      this.archer.isEmpty() &&
      this.axeman.isEmpty() &&
      this.heavyCavalry.isEmpty() &&
      this.lightCavalry.isEmpty() &&
      this.pikeman.isEmpty() &&
      this.swordsman.isEmpty()
    );
  }

  public static create(personnel: ArmyPersonnel) {
    return new ArmyPersonnelImpl(personnel);
  }

  public static size(personnel: ArmyPersonnel): ArmyPersonnelSize {
    return {
      archer: personnel.archer.size,
      axeman: personnel.axeman.size,
      heavyCavalry: personnel.heavyCavalry.size,
      lightCavalry: personnel.lightCavalry.size,
      pikeman: personnel.pikeman.size,
      swordsman: personnel.swordsman.size,
    };
  }
}
