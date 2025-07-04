// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface Village {
  readonly coord: Coord;
  readonly infrastructure: Infrastructure;
  readonly name: string;
  readonly owner: VillageOwner;
  readonly stability: number;
}

interface VillagePublicState {
  readonly coord: Coord;
  readonly name: string;
  readonly owner: VillageOwner;
}

interface Coord {
  readonly x: number;
  readonly y: number;
}

type VillageOwner = VillageOwnerNone | VillageOwnerPlayer;

interface VillageOwnerNone {
  readonly kind: 'none';
}

interface VillageOwnerPlayer {
  readonly id: PlayerId;
  readonly kind: 'player';
}
