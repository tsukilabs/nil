// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { ArmyImpl } from './army';
import { getPlayerMilitary } from '@/commands/player';
import { CoordImpl } from '@/core/model/continent/coord';
import { ManeuverImpl } from '@/core/model/military/maneuver';

export class MilitaryImpl implements Military {
  public readonly continent: ReadonlyMap<ContinentIndex, readonly ArmyImpl[]>;
  public readonly continentSize: ContinentSize;
  public readonly maneuvers: ReadonlyMap<ManeuverId, ManeuverImpl>;

  private constructor(args: Args) {
    this.continent = args.continent;
    this.continentSize = args.continentSize;
    this.maneuvers = args.maneuvers;
  }

  public getArmiesAt(key: ContinentKey): readonly ArmyImpl[] {
    if (typeof key !== 'number') {
      key = CoordImpl.toIndex(key);
    }

    return this.continent.get(key) ?? [];
  }

  public getIdleArmiesAt(key: ContinentKey): readonly ArmyImpl[] {
    return this.getArmiesAt(key).filter((army) => army.isIdle());
  }

  public getOwnIdleArmiesAt(key: ContinentKey): readonly ArmyImpl[] {
    const player = NIL.player.getId();
    return this.getIdleArmiesAt(key).filter(({ owner }) => {
      return owner.kind === 'player' && owner.id === player;
    });
  }

  public getManeuversBy(f: (maneuver: ManeuverImpl) => boolean) {
    return this.maneuvers.values()
      .filter((maneuver) => maneuver.isPending() && f(maneuver))
      .toArray();
  }

  public getGoingManeuversBy(f: (maneuver: ManeuverImpl) => boolean) {
    return this.getManeuversBy((maneuver) => {
      return maneuver.direction === 'going' && f(maneuver);
    });
  }

  public getReturningManeuversBy(f: (maneuver: ManeuverImpl) => boolean) {
    return this.getManeuversBy((maneuver) => {
      return maneuver.direction === 'returning' && f(maneuver);
    });
  }

  public getManeuversAt(key: ContinentKey) {
    return this.getManeuversBy((maneuver) => {
      return maneuver.origin.is(key) || maneuver.destination.is(key);
    });
  }

  public static fromRaw(raw: RawMilitary) {
    const continent = new Map<number, readonly ArmyImpl[]>();
    for (const [index, armies] of Object.entries(raw.continent)) {
      const impl = armies.map((army) => ArmyImpl.create(army));
      continent.set(Number.parseInt(index), impl);
    }

    const maneuvers = new Map<ManeuverId, ManeuverImpl>();
    for (const [id, maneuver] of Object.entries(raw.maneuvers)) {
      maneuvers.set(id, ManeuverImpl.create(maneuver));
    }

    return new MilitaryImpl({
      continent,
      continentSize: raw.continentSize,
      maneuvers,
    });
  }

  public static async load() {
    const military = await getPlayerMilitary();
    return MilitaryImpl.fromRaw(military);
  }
}

interface Args {
  continent: ReadonlyMap<number, readonly ArmyImpl[]>;
  continentSize: ContinentSize;
  maneuvers: ReadonlyMap<ManeuverId, ManeuverImpl>;
}

export interface RawMilitary {
  readonly continent: Readonly<Record<string, readonly Army[]>>;
  readonly continentSize: ContinentSize;
  readonly maneuvers: Readonly<Record<ManeuverId, Maneuver>>;
}
