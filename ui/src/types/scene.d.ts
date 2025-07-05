// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type Scene = 'home' | 'host-game' | 'join-game' | 'lobby' | 'settings' | GameScene;

type GameScene =
  | 'academy'
  | 'farm'
  | 'iron-mine'
  | 'map'
  | 'prefecture'
  | 'quarry'
  | 'sawmill'
  | 'script'
  | 'silo'
  | 'stable'
  | 'village'
  | 'wall'
  | 'warehouse';
