// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface PublicVillage {
  readonly coord: Coord;
  readonly name: string;
  readonly owner: VillageOwner;
}

interface Village extends PublicVillage {
  readonly infrastructure: Infrastructure;
  readonly stability: number;
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
  readonly kind: 'player';
  readonly id: PlayerId;
}
