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

  private constructor(args: {
    archer: SquadImpl;
    axeman: SquadImpl;
    heavyCavalry: SquadImpl;
    lightCavalry: SquadImpl;
    pikeman: SquadImpl;
    swordsman: SquadImpl;
  }) {
    this.archer = args.archer;
    this.axeman = args.axeman;
    this.heavyCavalry = args.heavyCavalry;
    this.lightCavalry = args.lightCavalry;
    this.pikeman = args.pikeman;
    this.swordsman = args.swordsman;
  }

  public static create(personnel: ArmyPersonnel) {
    return new ArmyPersonnelImpl({
      archer: SquadImpl.create(personnel.archer),
      axeman: SquadImpl.create(personnel.axeman),
      heavyCavalry: SquadImpl.create(personnel.heavyCavalry),
      lightCavalry: SquadImpl.create(personnel.lightCavalry),
      pikeman: SquadImpl.create(personnel.pikeman),
      swordsman: SquadImpl.create(personnel.swordsman),
    });
  }
}
