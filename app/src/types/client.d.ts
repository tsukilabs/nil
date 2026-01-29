// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface ClientOptions {
  serverAddr: ServerAddr;
  worldId?: Option<WorldId>;
  playerId?: Option<PlayerId>;
  playerPassword?: Option<string>;
  authorizationToken?: Option<string>;
}
