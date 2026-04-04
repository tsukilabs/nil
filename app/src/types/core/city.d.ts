// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface PublicCity {
  readonly coord: Coord;
  readonly name: string;
  readonly owner: Ruler;
}

interface City extends PublicCity {
  readonly infrastructure: Infrastructure;
  readonly stability: number;
}
