// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type Scene =
  | GameScene
  | OnlineScene
  | 'about'
  | 'home'
  | 'host-local-game'
  | 'join-local-game'
  | 'load-local-game'
  | 'settings';
