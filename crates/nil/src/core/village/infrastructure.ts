import * as building from '@/types/building';
import type { Infrastructure } from '@/types/village';

export class InfrastructureImpl implements Infrastructure {
  public readonly academy: building.Academy;
  public readonly farm: building.Farm;
  public readonly ironMine: building.IronMine;
  public readonly prefecture: building.Prefecture;
  public readonly quarry: building.Quarry;
  public readonly sawmill: building.Sawmill;
  public readonly stable: building.Stable;
  public readonly wall: building.Wall;
  public readonly warehouse: building.Warehouse;

  private constructor(infrastructure: Infrastructure) {
    this.academy = infrastructure.academy;
    this.farm = infrastructure.farm;
    this.ironMine = infrastructure.ironMine;
    this.prefecture = infrastructure.prefecture;
    this.quarry = infrastructure.quarry;
    this.sawmill = infrastructure.sawmill;
    this.stable = infrastructure.stable;
    this.wall = infrastructure.wall;
    this.warehouse = infrastructure.warehouse;
  }

  public static create(infrastructure: Infrastructure) {
    return new InfrastructureImpl(infrastructure);
  }
}
