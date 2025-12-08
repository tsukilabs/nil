// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type EventPayload =
  | ChatUpdatedPayload
  | CityUpdatedPayload
  | MilitaryUpdatedPayload
  | PlayerUpdatedPayload
  | PublicCityUpdatedPayload
  | ReportPayload
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

interface MilitaryUpdatedPayload {
  readonly kind: 'military-updated';
  readonly player: PlayerId;
}

interface PlayerUpdatedPayload {
  readonly kind: 'player-updated';
  readonly player: PlayerId;
}

interface PublicCityUpdatedPayload {
  readonly kind: 'public-city-updated';
  readonly coord: Coord;
}

interface ReportPayload {
  readonly kind: 'report';
  readonly report: ReportId;
}

interface RoundUpdatedPayload {
  readonly kind: 'round-updated';
  readonly round: Round;
}
