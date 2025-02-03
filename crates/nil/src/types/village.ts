import type { Coord } from './world';
import type { Option } from '@tb-dev/utils';
import type * as building from './building';

export interface Village {
  readonly coord: Coord;
  readonly infrastructure: Infrastructure;
  readonly name: string;
  readonly owner: Option<string>;
}

export interface Infrastructure {
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
}
