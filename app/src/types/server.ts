// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { RoundId } from '@/types/core/round';
import type { PlayerId } from '@/types/core/player';
import type { ContinentSize } from '@/types/core/continent';
import type { WorldConfig, WorldId } from '@/types/core/world';

export interface LocalServer {
  readonly worldId: WorldId;
  readonly addr: string;
}

export type ServerAddr = ServerAddrLocal | ServerAddrRemote;

export interface ServerAddrLocal {
  readonly kind: 'local';
  readonly addr: string;
}

export interface ServerAddrRemote {
  readonly kind: 'remote';
}

export type ServerKind = ServerKindLocal | ServerKindRemote;

export interface ServerKindLocal {
  readonly kind: 'local';
  readonly id: WorldId;
}

export interface ServerKindRemote {
  readonly kind: 'remote';
}

export interface RemoteWorld {
  readonly config: WorldConfig;
  readonly description: Option<string>;
  readonly createdBy: PlayerId;
  readonly createdAt: string;
  readonly updatedAt: string;
  readonly hasPassword: boolean;
  readonly currentRound: RoundId;
  readonly roundDuration: Option<RoundDuration>;
  readonly activePlayers: number;
  readonly totalPlayers: number;
  readonly continentSize: ContinentSize;
}

export type RoundDuration = number;
