// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type PublicField = PublicFieldEmpty | PublicFieldCity;

type PublicFieldKind = PublicField['kind'];

interface PublicFieldEmpty {
  readonly kind: 'empty';
}

interface PublicFieldCity {
  readonly kind: 'city';
  readonly city: PublicCity;
}

interface Coord {
  readonly x: number;
  readonly y: number;
}

type ContinentKey = Coord | ContinentIndex;
type ContinentIndex = number;
type ContinentSize = number;

interface CitySearch {
  coord?: Coord[];
  name?: string[];
}
