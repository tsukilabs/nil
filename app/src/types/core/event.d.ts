// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type EventPayload =
  | ChatUpdatedPayload
  | CityUpdatedPayload
  | DropPayload
  | MilitaryUpdatedPayload
  | PlayerUpdatedPayload
  | PublicCityUpdatedPayload
  | ReportPayload
  | RoundUpdatedPayload;

type EventPayloadKind = EventPayload['kind'];

interface ChatUpdatedPayload {
  readonly kind: 'chat-updated';
  readonly world: WorldId;
  readonly message: ChatMessage;
}

interface CityUpdatedPayload {
  readonly kind: 'city-updated';
  readonly world: WorldId;
  readonly coord: Coord;
}

interface DropPayload {
  readonly kind: 'drop';
  readonly world: WorldId;
}

interface MilitaryUpdatedPayload {
  readonly kind: 'military-updated';
  readonly world: WorldId;
  readonly player: PlayerId;
}

interface PlayerUpdatedPayload {
  readonly kind: 'player-updated';
  readonly world: WorldId;
  readonly player: PlayerId;
}

interface PublicCityUpdatedPayload {
  readonly kind: 'public-city-updated';
  readonly world: WorldId;
  readonly coord: Coord;
}

interface ReportPayload {
  readonly kind: 'report';
  readonly world: WorldId;
  readonly report: ReportId;
}

interface RoundUpdatedPayload {
  readonly kind: 'round-updated';
  readonly world: WorldId;
  readonly round: Round;
}
