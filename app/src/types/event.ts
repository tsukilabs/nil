// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Event } from "@tsukilabs/nil-bindings";

export type EventPayload = Event;

export type EventPayloadKind = EventPayload["kind"];

type KindMap = {
  [E in EventPayload as E["kind"]]: Readonly<E>;
};

export type ChatMessagePayload = KindMap["chat-message"];
export type CityPayload = KindMap["city"];
export type DropPayload = KindMap["drop"];
export type MilitaryPayload = KindMap["military"];
export type PlayerPayload = KindMap["player"];
export type PublicCityPayload = KindMap["public-city"];
export type ReportPayload = KindMap["report"];
export type RoundPayload = KindMap["round"];
