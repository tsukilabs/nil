// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { go } from '@/router';
import type { Option } from '@tb-dev/utils';
import type { Coord as WasmCoord } from '@tsukilabs/nil-continent';
import {
  QUERY_WAR_ROOM_DEST_X,
  QUERY_WAR_ROOM_DEST_Y,
  QUERY_WAR_ROOM_ORIGIN_X,
  QUERY_WAR_ROOM_ORIGIN_Y,
} from '@/router/game/war-room';

export class CoordImpl implements Coord {
  public readonly x: number;
  public readonly y: number;

  #id: Option<string>;
  #xOutside: Option<boolean>;
  #yOutside: Option<boolean>;
  #index: Option<ContinentIndex>;

  private constructor(coord: Coord) {
    // We should not use `toU8` or anything similar here,
    // because the `nil-continent` wasm code works with coordinates in the `i16` range.
    // Clamping the values to `u8` will break it.
    this.x = Math.trunc(coord.x);
    this.y = Math.trunc(coord.y);
  }

  public is(other: ContinentKey) {
    other = CoordImpl.fromKey(other);
    return this.x === other.x && this.y === other.y;
  }

  public isOutside() {
    return this.isXOutside() || this.isYOutside();
  }

  public isXOutside() {
    if (typeof this.#xOutside !== 'boolean') {
      this.#xOutside = isOutside(this.x);
    }

    return this.#xOutside;
  }

  public isYOutside() {
    if (typeof this.#yOutside !== 'boolean') {
      this.#yOutside = isOutside(this.y);
    }

    return this.#yOutside;
  }

  private async goWithQuery(scene: Scene, queryX = 'x', queryY = 'y') {
    const x = this.x.toString(10);
    const y = this.y.toString(10);
    await go(scene, {
      query: { [queryX]: x, [queryY]: y },
    });
  }

  public async goToContinent() {
    await this.goWithQuery('continent');
  }

  public async goToProfile() {
    const ckey = this.toIndexString();
    await go('profile-city', { params: { ckey } });
  }

  public async goToWarRoom(kind: 'origin' | 'destination') {
    const queryX = kind === 'origin' ? QUERY_WAR_ROOM_ORIGIN_X : QUERY_WAR_ROOM_DEST_X;
    const queryY = kind === 'origin' ? QUERY_WAR_ROOM_ORIGIN_Y : QUERY_WAR_ROOM_DEST_Y;
    await this.goWithQuery('war-room', queryX, queryY);
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

  public toIndex() {
    this.#index ??= CoordImpl.toIndex(this);
    return this.#index;
  }

  public toIndexString() {
    return this.toIndex().toString(10);
  }

  public toJSON() {
    return { x: this.x, y: this.y };
  }

  get id() {
    return this.format();
  }

  public static create(coord: Coord) {
    if (coord instanceof CoordImpl) {
      return coord;
    }

    return new CoordImpl(coord);
  }

  public static splat(value: number) {
    return CoordImpl.create({ x: value, y: value });
  }

  public static fromKey(key: ContinentKey) {
    if (typeof key === 'number') {
      return CoordImpl.fromIndex(key);
    }

    return CoordImpl.create(key);
  }

  public static fromIndex(index: ContinentIndex) {
    const size = NIL.world.getContinentSize();
    const x = index % size;
    const y = index / size;
    return CoordImpl.create({ x, y });
  }

  public static toIndex(coord: Coord) {
    const size = NIL.world.getContinentSize();
    return coord.y * size + coord.x;
  }

  public static fromWasm(coord: WasmCoord) {
    return CoordImpl.create({ x: coord.x(), y: coord.y() });
  }

  private static readonly intl = new Intl.NumberFormat('default', {
    maximumFractionDigits: 0,
    minimumIntegerDigits: 3,
    style: 'decimal',
    useGrouping: false,
  });
}

export function isOutside(value: number, continentSize?: number) {
  const size = continentSize ?? NIL.world.getContinentSize();
  return !Number.isInteger(value) || value < 0 || value >= size;
}
