// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type Scene = 'home' | 'host-game' | 'join-game' | 'lobby' | 'settings' | GameScene;

type GameScene = 'continent' | 'script' | 'village' | InfrastructureScene;

type InfrastructureScene =
  | 'academy'
  | 'farm'
  | 'iron-mine'
  | 'prefecture'
  | 'quarry'
  | 'sawmill'
  | 'silo'
  | 'stable'
  | 'wall'
  | 'warehouse';
