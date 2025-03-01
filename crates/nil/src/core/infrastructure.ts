export class InfrastructureImpl implements Infrastructure {
  public readonly academy: Academy;
  public readonly farm: Farm;
  public readonly ironMine: IronMine;
  public readonly prefecture: Prefecture;
  public readonly quarry: Quarry;
  public readonly sawmill: Sawmill;
  public readonly silo: Silo;
  public readonly stable: Stable;
  public readonly wall: Wall;
  public readonly warehouse: Warehouse;

  private constructor(infrastructure: Infrastructure) {
    this.academy = infrastructure.academy;
    this.farm = infrastructure.farm;
    this.ironMine = infrastructure.ironMine;
    this.prefecture = infrastructure.prefecture;
    this.quarry = infrastructure.quarry;
    this.sawmill = infrastructure.sawmill;
    this.silo = infrastructure.silo;
    this.stable = infrastructure.stable;
    this.wall = infrastructure.wall;
    this.warehouse = infrastructure.warehouse;
  }

  public static create(infrastructure: Infrastructure) {
    return new InfrastructureImpl(infrastructure);
  }
}
