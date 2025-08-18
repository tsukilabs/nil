// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface PublicCity {
  readonly coord: Coord;
  readonly name: string;
  readonly owner: CityOwner;
}

interface City extends PublicCity {
  readonly infrastructure: Infrastructure;
  readonly stability: number;
}

type CityOwner = CityOwnerBot | CityOwnerPlayer | CityOwnerPrecursor;

type CityOwnerId =
  | CityOwnerBot['id']
  | CityOwnerPlayer['id']
  | CityOwnerPrecursor['id'];

type CityOwnerKind =
  | CityOwnerBot['kind']
  | CityOwnerPlayer['kind']
  | CityOwnerPrecursor['kind'];

interface CityOwnerBot {
  readonly kind: 'bot';
  readonly id: BotId;
}

interface CityOwnerPlayer {
  readonly kind: 'player';
  readonly id: PlayerId;
}

interface CityOwnerPrecursor {
  readonly kind: 'precursor';
  readonly id: PrecursorId;
}
