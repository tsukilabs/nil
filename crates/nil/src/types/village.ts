import type { Option } from '@tb-dev/utils';
import type * as building from './building';

export type Village = {
  readonly coord: Coord;
  readonly infrastructure: Infrastructure;
  readonly name: string;
  readonly owner: Option<string>;
};

export type Infrastructure = {
  readonly academy: building.Academy;
  readonly farm: building.Farm;
  readonly ironMine: building.IronMine;
  readonly prefecture: building.Prefecture;
  readonly quarry: building.Quarry;
  readonly sawmill: building.Sawmill;
  readonly silo: building.Silo;
  readonly stable: building.Stable;
  readonly wall: building.Wall;
  readonly warehouse: building.Warehouse;
};

export type Coord = {
  readonly x: number;
  readonly y: number;
};
