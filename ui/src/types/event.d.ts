// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type EventPayload =
  | ChatUpdatedPayload
  | CityUpdatedPayload
  | PlayerUpdatedPayload
  | PublicCityUpdatedPayload
  | RoundUpdatedPayload;

type EventPayloadKind = EventPayload['kind'];

interface ChatUpdatedPayload {
  readonly kind: 'chat-updated';
  readonly message: ChatMessage;
}

interface CityUpdatedPayload {
  readonly kind: 'city-updated';
  readonly coord: Coord;
}

interface PlayerUpdatedPayload {
  readonly kind: 'player-updated';
  readonly player: PlayerId;
}

interface PublicCityUpdatedPayload {
  readonly kind: 'public-city-updated';
  readonly coord: Coord;
}

interface RoundUpdatedPayload {
  readonly kind: 'round-updated';
  readonly round: Round;
}
