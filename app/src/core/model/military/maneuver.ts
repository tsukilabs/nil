// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { go } from "@/router";
import * as commands from "@/commands";
import { getManeuver } from "@/commands";
import type { Option } from "@tb-dev/utils";
import { CoordImpl } from "@/core/model/continent/coord";
import { PublicCityImpl } from "@/core/model/city/public-city";
import { ManeuverHaulImpl } from "@/core/model/military/maneuver-haul";
import type {
  ArmyId,
  Maneuver,
  ManeuverDirection,
  ManeuverId,
  ManeuverKind,
  ManeuverState,
  Ruler,
} from "@tsukilabs/nil-bindings";

export class ManeuverImpl implements Readonly<Maneuver> {
  public readonly id: ManeuverId;
  public readonly origin: CoordImpl;
  public readonly destination: CoordImpl;
  public readonly army: ArmyId;
  public readonly kind: ManeuverKind;
  public readonly direction: ManeuverDirection;
  public readonly state: ManeuverState;
  public readonly speed: number;
  public readonly hauledResources: ManeuverHaulImpl | null;

  #armyOwner: Option<Ruler>;
  #originCity: Option<PublicCityImpl>;
  #destinationCity: Option<PublicCityImpl>;

  private constructor(maneuver: Maneuver) {
    this.id = maneuver.id;
    this.origin = CoordImpl.create(maneuver.origin);
    this.destination = CoordImpl.create(maneuver.destination);
    this.army = maneuver.army;
    this.kind = maneuver.kind;
    this.direction = maneuver.direction;
    this.state = maneuver.state;
    this.speed = maneuver.speed;

    this.hauledResources = maneuver.hauledResources ?
      ManeuverHaulImpl.create(maneuver.hauledResources) :
      null;
  }

  public isAttack() {
    return this.kind === "attack";
  }

  public isSupport() {
    return this.kind === "support";
  }

  public isGoing() {
    return this.direction === "going";
  }

  public isReturning() {
    return this.direction === "returning";
  }

  public isDone() {
    return this.state.kind === "done";
  }

  public isPending() {
    return this.state.kind === "pending";
  }

  public getPendingDistance() {
    if (this.state.kind === "pending") {
      return this.state.distance;
    }
    else {
      return 0;
    }
  }

  public async goToManeuverScene() {
    await go("maneuver", {
      params: { id: this.id },
    });
  }

  public hasHauledResources() {
    if (this.hauledResources) {
      return !this.hauledResources.isEmpty();
    }
    else {
      return false;
    }
  }

  public async getArmyOwner(): Promise<Ruler> {
    this.#armyOwner ??= await commands.getArmyOwner(this.army);
    return this.#armyOwner;
  }

  public async getCities() {
    if (!this.#originCity || !this.#destinationCity) {
      const cities = await PublicCityImpl.bulkLoad([this.origin, this.destination]);
      this.#originCity = cities.find((city) => city.coord.is(this.origin)) ?? null;
      this.#destinationCity = cities.find((city) => city.coord.is(this.destination)) ?? null;
    }

    return {
      origin: this.#originCity,
      destination: this.#destinationCity,
    } as const;
  }

  public static create(maneuver: Maneuver) {
    if (maneuver instanceof ManeuverImpl) {
      return maneuver;
    }

    return new ManeuverImpl(maneuver);
  }

  public static async load(id: ManeuverId) {
    const maneuver = await getManeuver(id);
    return maneuver ? ManeuverImpl.create(maneuver) : null;
  }
}
