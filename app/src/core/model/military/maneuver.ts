// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { CoordImpl } from '@/core/model/continent/coord';
import { ManeuverHaulImpl } from '@/core/model/military/maneuver-haul';

export class ManeuverImpl implements Maneuver {
  public readonly id: ManeuverId;
  public readonly origin: CoordImpl;
  public readonly destination: CoordImpl;
  public readonly army: ArmyId;
  public readonly kind: ManeuverKind;
  public readonly direction: ManeuverDirection;
  public readonly state: ManeuverState;
  public readonly hauledResources: Option<ManeuverHaulImpl>;

  private constructor(maneuver: Maneuver) {
    this.id = maneuver.id;
    this.origin = CoordImpl.create(maneuver.origin);
    this.destination = CoordImpl.create(maneuver.destination);
    this.army = maneuver.army;
    this.kind = maneuver.kind;
    this.direction = maneuver.direction;
    this.state = maneuver.state;

    this.hauledResources = maneuver.hauledResources ?
      ManeuverHaulImpl.create(maneuver.hauledResources) :
      null;
  }

  public isAttack() {
    return this.kind === 'attack';
  }

  public isSupport() {
    return this.kind === 'support';
  }

  public isGoing() {
    return this.direction === 'going';
  }

  public isReturning() {
    return this.direction === 'returning';
  }

  public isDone() {
    return this.state.kind === 'done';
  }

  public isPending() {
    return this.state.kind === 'pending';
  }

  public getPendingDistance() {
    if (this.state.kind === 'pending') {
      return this.state.distance;
    }
    else {
      return 0;
    }
  }

  public static create(maneuver: Maneuver) {
    if (maneuver instanceof ManeuverImpl) {
      return maneuver;
    }

    return new ManeuverImpl(maneuver);
  }
}
