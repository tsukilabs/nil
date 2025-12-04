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

  public *[Symbol.iterator]() {
    yield* this.getSquads();
  }

  public getSquads() {
    return [
      this.archer,
      this.axeman,
      this.heavyCavalry,
      this.lightCavalry,
      this.pikeman,
      this.swordsman,
    ];
  }

  public size() {
    return ArmyPersonnelImpl.size(this);
  }

  public isEmpty() {
    return ArmyPersonnelImpl.isEmpty(this);
  }

  public normalize() {
    return ArmyPersonnelImpl.normalize(this);
  }

  public add(rhs: ArmyPersonnel) {
    return ArmyPersonnelImpl.add(this, rhs);
  }

  public sub(rhs: ArmyPersonnel) {
    return ArmyPersonnelImpl.sub(this, rhs);
  }

  public static create(personnel: ArmyPersonnel) {
    if (personnel instanceof ArmyPersonnelImpl) {
      return personnel;
    }

    return new ArmyPersonnelImpl(personnel);
  }

  public static createEmpty() {
    return this.create(ArmyPersonnelImpl.createEmptyRaw());
  }

  public static createEmptyRaw(): ArmyPersonnel {
    return {
      archer: SquadImpl.createEmptyRaw('archer'),
      axeman: SquadImpl.createEmptyRaw('axeman'),
      pikeman: SquadImpl.createEmptyRaw('pikeman'),
      swordsman: SquadImpl.createEmptyRaw('swordsman'),
      heavyCavalry: SquadImpl.createEmptyRaw('heavy-cavalry'),
      lightCavalry: SquadImpl.createEmptyRaw('light-cavalry'),
    };
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

  public static isEmpty(personnel: ArmyPersonnel) {
    return (
      SquadImpl.isEmpty(personnel.archer) &&
      SquadImpl.isEmpty(personnel.axeman) &&
      SquadImpl.isEmpty(personnel.heavyCavalry) &&
      SquadImpl.isEmpty(personnel.lightCavalry) &&
      SquadImpl.isEmpty(personnel.pikeman) &&
      SquadImpl.isEmpty(personnel.swordsman)
    );
  }

  public static normalize(personnel: ArmyPersonnel) {
    return ArmyPersonnelImpl.create({
      archer: SquadImpl.normalize(personnel.archer),
      axeman: SquadImpl.normalize(personnel.axeman),
      heavyCavalry: SquadImpl.normalize(personnel.heavyCavalry),
      lightCavalry: SquadImpl.normalize(personnel.lightCavalry),
      pikeman: SquadImpl.normalize(personnel.pikeman),
      swordsman: SquadImpl.normalize(personnel.swordsman),
    });
  }

  public static add(lhs: ArmyPersonnel, rhs: ArmyPersonnel) {
    return ArmyPersonnelImpl.create({
      archer: SquadImpl.add(lhs.archer, rhs.archer),
      axeman: SquadImpl.add(lhs.axeman, rhs.axeman),
      heavyCavalry: SquadImpl.add(lhs.heavyCavalry, rhs.heavyCavalry),
      lightCavalry: SquadImpl.add(lhs.lightCavalry, rhs.lightCavalry),
      pikeman: SquadImpl.add(lhs.pikeman, rhs.pikeman),
      swordsman: SquadImpl.add(lhs.swordsman, rhs.swordsman),
    });
  }

  public static sub(lhs: ArmyPersonnel, rhs: ArmyPersonnel) {
    return ArmyPersonnelImpl.create({
      archer: SquadImpl.sub(lhs.archer, rhs.archer),
      axeman: SquadImpl.sub(lhs.axeman, rhs.axeman),
      heavyCavalry: SquadImpl.sub(lhs.heavyCavalry, rhs.heavyCavalry),
      lightCavalry: SquadImpl.sub(lhs.lightCavalry, rhs.lightCavalry),
      pikeman: SquadImpl.sub(lhs.pikeman, rhs.pikeman),
      swordsman: SquadImpl.sub(lhs.swordsman, rhs.swordsman),
    });
  }
}
