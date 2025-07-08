// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type EventPayload =
  | ChatMessagePayload
  | GuestLeftPayload
  | LobbyUpdatedPayload
  | PlayerSpawnedPayload
  | PlayerStatusUpdatedPayload
  | PlayerUpdatedPayload
  | RoundUpdatedPayload
  | VillageSpawnedPayload
  | VillageUpdatedPayload;

type EventPayloadKind = EventPayload['kind'];

interface ChatMessagePayload {
  readonly kind: 'chat-message';
  readonly message: ChatMessage;
}

interface GuestLeftPayload {
  readonly kind: 'guest-left';
  readonly guest: Player;
}

interface LobbyUpdatedPayload {
  readonly kind: 'lobby-updated';
  readonly lobby: Lobby;
}

interface PlayerSpawnedPayload {
  readonly kind: 'player-spawned';
  readonly player: Player;
}

interface PlayerStatusUpdatedPayload {
  readonly kind: 'player-status-updated';
  readonly player: PlayerId;
  readonly status: PlayerStatus;
}

interface PlayerUpdatedPayload {
  readonly kind: 'player-updated';
  readonly player: PlayerId;
}

interface RoundUpdatedPayload {
  readonly kind: 'round-updated';
  readonly round: Round;
}

interface VillageSpawnedPayload {
  readonly kind: 'village-spawned';
  readonly village: PublicVillage;
}

interface VillageUpdatedPayload {
  readonly kind: 'village-updated';
  readonly coord: Coord;
}
