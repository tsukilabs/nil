import * as commands from '@/commands';
import { CoordImpl } from '@/core/coord';
import { InfrastructureImpl } from '@/core/infrastructure';

export class VillageImpl implements Village {
  public readonly coord: CoordImpl;
  public readonly infrastructure: InfrastructureImpl;
  public readonly name: string;
  public readonly owner: VillageOwner;

  private constructor(village: Village) {
    this.coord = CoordImpl.create(village.coord);
    this.infrastructure = InfrastructureImpl.create(village.infrastructure);
    this.name = village.name;
    this.owner = village.owner;
  }

  public static async load(coord: Coord) {
    const village = await commands.getVillage(coord);
    return new VillageImpl(village);
  }

  get academy() {
    return this.infrastructure.academy;
  }

  get farm() {
    return this.infrastructure.farm;
  }

  get ironMine() {
    return this.infrastructure.ironMine;
  }

  get prefecture() {
    return this.infrastructure.prefecture;
  }

  get quarry() {
    return this.infrastructure.quarry;
  }

  get sawmill() {
    return this.infrastructure.sawmill;
  }

  get silo() {
    return this.infrastructure.silo;
  }

  get stable() {
    return this.infrastructure.stable;
  }

  get wall() {
    return this.infrastructure.wall;
  }

  get warehouse() {
    return this.infrastructure.warehouse;
  }
}
