type Village = {
  readonly coord: Coord;
  readonly infrastructure: Infrastructure;
  readonly name: string;
  readonly owner: VillageOwner;
};

type Infrastructure = {
  readonly academy: Academy;
  readonly farm: Farm;
  readonly ironMine: IronMine;
  readonly prefecture: Prefecture;
  readonly quarry: Quarry;
  readonly sawmill: Sawmill;
  readonly silo: Silo;
  readonly stable: Stable;
  readonly wall: Wall;
  readonly warehouse: Warehouse;
};

type Coord = {
  readonly x: number;
  readonly y: number;
};

type VillageOwner = VillageOwnerNone | VillageOwnerPlayer;

type VillageOwnerNone = {
  readonly kind: 'none';
};

type VillageOwnerPlayer = {
  readonly id: PlayerId;
  readonly kind: 'player';
};
