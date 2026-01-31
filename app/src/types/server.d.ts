// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface LocalServer {
  readonly worldId: WorldId;
  readonly addr: string;
}

type ServerAddr = ServerAddrLocal | ServerAddrRemote;

interface ServerAddrLocal {
  readonly kind: 'local';
  readonly addr: string;
}

interface ServerAddrRemote {
  readonly kind: 'remote';
}

type ServerKind = ServerKindLocal | ServerKindRemote;

interface ServerKindLocal {
  readonly kind: 'local';
  readonly id: WorldId;
}

interface ServerKindRemote {
  readonly kind: 'remote';
}

interface RemoteWorld {
  readonly config: WorldConfig;
  readonly description: Option<string>;
  readonly createdBy: PlayerId;
  readonly createdAt: string;
  readonly updatedAt: string;
  readonly hasPassword: boolean;
  readonly currentRound: RoundId;
  readonly activePlayers: number;
  readonly totalPlayers: number;
  readonly continentSize: ContinentSize;
}
