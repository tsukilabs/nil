// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Round } from '@/types/core/round';
import type { WorldId } from '@/types/core/world';
import type { Coord } from '@/types/core/continent';
import type { PlayerId } from '@/types/core/player';
import type { ReportId } from '@/types/core/report';
import type { ChatMessage } from '@/types/core/chat';

export type EventPayload =
  | ChatUpdatedPayload
  | CityUpdatedPayload
  | DropPayload
  | MilitaryUpdatedPayload
  | PlayerUpdatedPayload
  | PublicCityUpdatedPayload
  | ReportPayload
  | RoundUpdatedPayload;

export type EventPayloadKind = EventPayload['kind'];

export interface ChatUpdatedPayload {
  readonly kind: 'chat-updated';
  readonly world: WorldId;
  readonly message: ChatMessage;
}

export interface CityUpdatedPayload {
  readonly kind: 'city-updated';
  readonly world: WorldId;
  readonly coord: Coord;
}

export interface DropPayload {
  readonly kind: 'drop';
  readonly world: WorldId;
}

export interface MilitaryUpdatedPayload {
  readonly kind: 'military-updated';
  readonly world: WorldId;
  readonly player: PlayerId;
}

export interface PlayerUpdatedPayload {
  readonly kind: 'player-updated';
  readonly world: WorldId;
  readonly player: PlayerId;
}

export interface PublicCityUpdatedPayload {
  readonly kind: 'public-city-updated';
  readonly world: WorldId;
  readonly coord: Coord;
}

export interface ReportPayload {
  readonly kind: 'report';
  readonly world: WorldId;
  readonly report: ReportId;
}

export interface RoundUpdatedPayload {
  readonly kind: 'round-updated';
  readonly world: WorldId;
  readonly round: Round;
}
