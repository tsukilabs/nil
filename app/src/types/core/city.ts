// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export interface PublicCity {
  readonly coord: Coord;
  readonly name: string;
  readonly owner: Ruler;
}

export interface City extends PublicCity {
  readonly infrastructure: Infrastructure;
  readonly stability: number;
}
