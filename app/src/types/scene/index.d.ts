// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type Scene =
  | GameScene
  | 'about'
  | 'home'
  | 'host-local-game'
  | 'host-remote-game'
  | 'join-local-game'
  | 'join-remote-game'
  | 'load-local-game'
  | 'lobby'
  | 'settings'
  | 'sign-in'
  | 'sign-up';
