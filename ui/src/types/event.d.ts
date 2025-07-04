// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type EventPayload =
  | ChatMessagePayload
  | FailedToSaveWorldPayload
  | GuestLeftPayload
  | LobbyUpdatedPayload
  | PlayerResourcesUpdatedPayload
  | PlayerSpawnedPayload
  | PlayerStatusUpdatedPayload
  | PrefectureBuildQueueUpdatedPayload
  | RoundUpdatedPayload
  | VillageSpawnedPayload;

interface ChatMessagePayload {
  readonly kind: 'chat-message';
  readonly message: ChatMessage;
}

interface FailedToSaveWorldPayload {
  readonly kind: 'failed-to-save-world';
  readonly error: string;
}

interface GuestLeftPayload {
  readonly kind: 'guest-left';
  readonly guest: Player;
}

interface LobbyUpdatedPayload {
  readonly kind: 'lobby-updated';
  readonly lobby: Lobby;
}

interface PlayerResourcesUpdatedPayload {
  readonly kind: 'player-resources-updated';
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

interface PrefectureBuildQueueUpdatedPayload {
  readonly kind: 'prefecture-build-queue-updated';
  readonly coord: Coord;
  readonly id: PrefectureBuildOrderId;
  readonly orderKind: PrefectureBuildOrderKind;
}

interface RoundUpdatedPayload {
  readonly kind: 'round-updated';
  readonly round: Round;
}

interface VillageSpawnedPayload {
  readonly kind: 'village-spawned';
  readonly village: VillagePublicState;
}
