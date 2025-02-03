import type { Coord } from '@/types/world';

export class CoordImpl implements Coord {
  public readonly x: number;
  public readonly y: number;

  private constructor(coord: Coord) {
    this.x = coord.x;
    this.y = coord.y;
  }

  public is(other: Coord) {
    return this.x === other.x && this.y === other.y;
  }

  public format() {
    return `${CoordImpl.intl.format(this.x)}|${CoordImpl.intl.format(this.y)}`;
  }

  public static create(coord: Coord) {
    return new CoordImpl(coord);
  }

  private static readonly intl = new Intl.NumberFormat('default', {
    minimumIntegerDigits: 3,
    style: 'decimal',
    useGrouping: false,
  });
}
