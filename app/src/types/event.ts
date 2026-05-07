// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Event } from '@/types/bindings';

export type EventPayload = Event;

export type EventPayloadKind = EventPayload['kind'];

type KindMap = {
  [E in EventPayload as E['kind']]: Readonly<E>;
};

export type ChatUpdatedPayload = KindMap['chat-updated'];
export type CityUpdatedPayload = KindMap['city-updated'];
export type DropPayload = KindMap['drop'];
export type MilitaryUpdatedPayload = KindMap['military-updated'];
export type PlayerUpdatedPayload = KindMap['player-updated'];
export type PublicCityUpdatedPayload = KindMap['public-city-updated'];
export type ReportPayload = KindMap['report'];
export type RoundUpdatedPayload = KindMap['round-updated'];
