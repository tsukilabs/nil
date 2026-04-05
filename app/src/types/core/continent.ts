// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { PublicCity } from '@/types/core/city';

export type PublicField = PublicFieldEmpty | PublicFieldCity;

export type PublicFieldKind = PublicField['kind'];

export interface PublicFieldEmpty {
  readonly kind: 'empty';
}

export interface PublicFieldCity {
  readonly kind: 'city';
  readonly city: PublicCity;
}

export interface Coord {
  readonly x: number;
  readonly y: number;
}

export type CoordTuple = [Coord['x'], Coord['y']];

export type ContinentKey = Coord | CoordTuple | ContinentIndex;
export type ContinentIndex = number;
export type ContinentSize = number;

export interface CitySearch {
  coord?: Coord[];
  name?: string[];
}
