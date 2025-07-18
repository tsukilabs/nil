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

type VillageOwner = VillageOwnerBot | VillageOwnerPlayer | VillageOwnerPrecursor;

interface VillageOwnerBot {
  readonly kind: 'bot';
  readonly id: BotId;
}

interface VillageOwnerPlayer {
  readonly kind: 'player';
  readonly id: PlayerId;
}

interface VillageOwnerPrecursor {
  readonly kind: 'precursor';
  readonly id: PrecursorId;
}
