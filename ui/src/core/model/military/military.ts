// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { ArmyImpl } from './army';
import { getPlayerMilitary } from '@/commands/player';
import { CoordImpl } from '@/core/model/continent/coord';

export class MilitaryImpl implements Military {
  public readonly continent: ReadonlyMap<number, readonly ArmyImpl[]>;
  public readonly continentSize: ContinentSize;

  private constructor(args: Args) {
    this.continent = args.continent;
    this.continentSize = args.continentSize;
  }

  public getArmiesAt(key: ContinentKey): readonly ArmyImpl[] {
    if (typeof key !== 'number') {
      key = CoordImpl.toIndex(key);
    }

    return this.continent.get(key) ?? [];
  }

  public getIdleArmiesAt(key: ContinentKey): readonly ArmyImpl[] {
    return this.getArmiesAt(key).filter((army) => !army.isIdle());
  }

  public static fromRaw(raw: RawMilitary) {
    const continent = new Map<number, readonly ArmyImpl[]>();
    for (const [index, armies] of Object.entries(raw.continent)) {
      const impl = armies.map((army) => ArmyImpl.create(army));
      continent.set(Number.parseInt(index), impl);
    }

    return new MilitaryImpl({
      continent,
      continentSize: raw.continentSize,
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
}

export interface RawMilitary {
  readonly continent: Readonly<Record<string, readonly Army[]>>;
  readonly continentSize: ContinentSize;
}
