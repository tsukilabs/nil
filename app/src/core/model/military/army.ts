// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { getArmy } from "@/commands";
import { ArmyPersonnelImpl } from "./army-personnel";
import type { Army, ArmyId, ArmyState, Ruler } from "@tsukilabs/nil-bindings";

export class ArmyImpl implements Readonly<Army> {
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
    return this.state.kind === "idle";
  }

  public isManeuvering() {
    return this.state.kind === "maneuvering";
  }

  public isEmpty() {
    return this.personnel.isEmpty();
  }

  get archer() {
    return this.personnel.archer;
  }

  get axeman() {
    return this.personnel.axeman;
  }

  get heavyCavalry() {
    return this.personnel.heavyCavalry;
  }

  get lightCavalry() {
    return this.personnel.lightCavalry;
  }

  get pikeman() {
    return this.personnel.pikeman;
  }

  get ram() {
    return this.personnel.ram;
  }

  get swordsman() {
    return this.personnel.swordsman;
  }

  public static create(army: Army) {
    if (army instanceof ArmyImpl) {
      return army;
    }

    return new ArmyImpl(army);
  }

  public static async load(id: ArmyId) {
    const army = await getArmy(id);
    return ArmyImpl.create(army);
  }
}
