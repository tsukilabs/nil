// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { FarmImpl } from './building/farm';
import { SiloImpl } from './building/silo';
import { WallImpl } from './building/wall';
import { QuarryImpl } from './building/quarry';
import { SawmillImpl } from './building/sawmill';
import { IronMineImpl } from './building/iron-mine';
import { WarehouseImpl } from './building/warehouse';
import { StableImpl } from './building/stable/stable';
import { AcademyImpl } from './building/academy/academy';
import { PrefectureImpl } from './building/prefecture/prefecture';

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

  private constructor(infrastructure: Infrastructure) {
    this.academy = AcademyImpl.create(infrastructure.academy);
    this.farm = FarmImpl.create(infrastructure.farm);
    this.ironMine = IronMineImpl.create(infrastructure.ironMine);
    this.prefecture = PrefectureImpl.create(infrastructure.prefecture);
    this.quarry = QuarryImpl.create(infrastructure.quarry);
    this.sawmill = SawmillImpl.create(infrastructure.sawmill);
    this.silo = SiloImpl.create(infrastructure.silo);
    this.stable = StableImpl.create(infrastructure.stable);
    this.wall = WallImpl.create(infrastructure.wall);
    this.warehouse = WarehouseImpl.create(infrastructure.warehouse);
  }

  public static create(infrastructure: Infrastructure) {
    return new InfrastructureImpl(infrastructure);
  }
}
