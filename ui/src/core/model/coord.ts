// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Option } from '@tb-dev/utils';

export class CoordImpl implements Coord {
  public readonly x: number;
  public readonly y: number;

  #id: Option<string>;
  #outside: Option<boolean>;

  private constructor(coord: Coord) {
    this.x = coord.x;
    this.y = coord.y;
  }

  public is(other: Coord) {
    return this.x === other.x && this.y === other.y;
  }

  public isOutside(size: number) {
    if (typeof this.#outside !== 'boolean') {
      this.#outside = this.isXOutside(size) || this.isYOutside(size);
    }

    return this.#outside;
  }

  public isXOutside(size: number) {
    return this.x < 0 || this.x >= size;
  }

  public isYOutside(size: number) {
    return this.y < 0 || this.y >= size;
  }

  public format() {
    this.#id ??= `${this.formatX()}|${this.formatY()}`;
    return this.#id;
  }

  public formatX() {
    return CoordImpl.intl.format(this.x);
  }

  public formatY() {
    return CoordImpl.intl.format(this.y);
  }

  public toJSON() {
    return { x: this.x, y: this.y };
  }

  get id() {
    return this.format();
  }

  public static create(coord: Coord) {
    return new CoordImpl(coord);
  }

  private static readonly intl = new Intl.NumberFormat('default', {
    maximumFractionDigits: 0,
    minimumIntegerDigits: 3,
    style: 'decimal',
    useGrouping: false,
  });
}
