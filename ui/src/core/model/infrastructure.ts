// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { FarmImpl } from './buildings/farm';
import { SiloImpl } from './buildings/silo';
import { WallImpl } from './buildings/wall';
import { QuarryImpl } from './buildings/quarry';
import { StableImpl } from './buildings/stable';
import { AcademyImpl } from './buildings/academy';
import { SawmillImpl } from './buildings/sawmill';
import { IronMineImpl } from './buildings/iron-mine';
import { WarehouseImpl } from './buildings/warehouse';
import { PrefectureImpl } from './buildings/prefecture';

export class InfrastructureImpl implements Infrastructure {
  public readonly academy: AcademyImpl;
  public readonly farm: FarmImpl;
  public readonly ironMine: IronMineImpl;
  public readonly prefecture: PrefectureImpl;
  public readonly quarry: QuarryImpl;
  public readonly sawmill: SawmillImpl;
  public readonly silo: SiloImpl;
  public readonly stable: StableImpl;
  public readonly wall: WallImpl;
  public readonly warehouse: WarehouseImpl;

  private constructor(args: {
    academy: AcademyImpl;
    farm: FarmImpl;
    ironMine: IronMineImpl;
    prefecture: PrefectureImpl;
    quarry: QuarryImpl;
    sawmill: SawmillImpl;
    silo: SiloImpl;
    stable: StableImpl;
    wall: WallImpl;
    warehouse: WarehouseImpl;
  }) {
    this.academy = args.academy;
    this.farm = args.farm;
    this.ironMine = args.ironMine;
    this.prefecture = args.prefecture;
    this.quarry = args.quarry;
    this.sawmill = args.sawmill;
    this.silo = args.silo;
    this.stable = args.stable;
    this.wall = args.wall;
    this.warehouse = args.warehouse;
  }

  public static create(infrastructure: Infrastructure) {
    return new InfrastructureImpl({
      academy: AcademyImpl.create(infrastructure.academy),
      farm: FarmImpl.create(infrastructure.farm),
      ironMine: IronMineImpl.create(infrastructure.ironMine),
      prefecture: PrefectureImpl.create(infrastructure.prefecture),
      quarry: QuarryImpl.create(infrastructure.quarry),
      sawmill: SawmillImpl.create(infrastructure.sawmill),
      silo: SiloImpl.create(infrastructure.silo),
      stable: StableImpl.create(infrastructure.stable),
      wall: WallImpl.create(infrastructure.wall),
      warehouse: WarehouseImpl.create(infrastructure.warehouse),
    });
  }
}
