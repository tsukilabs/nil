import type { Coord } from '@/types/world';
import type { Option } from '@tb-dev/utils';
import type { Village } from '@/types/village';
import { InfrastructureImpl } from './infrastructure';

export class VillageImpl implements Village {
  public readonly coord: Coord;
  public readonly infrastructure: InfrastructureImpl;
  public readonly name: string;
  public readonly owner: Option<string>;

  private constructor(village: Village) {
    this.coord = village.coord;
    this.infrastructure = InfrastructureImpl.create(village.infrastructure);
    this.name = village.name;
    this.owner = village.owner;
  }

  public static create(village: Village) {
    return new VillageImpl(village);
  }
}
