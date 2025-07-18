// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type Scene = 'home' | 'host-game' | 'join-game' | 'settings' | GameScene;

type GameScene = 'continent' | 'village' | InfrastructureScene | ScriptScene;

type InfrastructureScene =
  | 'academy'
  | 'farm'
  | 'iron-mine'
  | 'quarry'
  | 'sawmill'
  | 'silo'
  | 'stable'
  | 'wall'
  | 'warehouse'
  | PrefectureScene;

type PrefectureScene = 'prefecture' | 'village-management';

type ScriptScene = 'script' | 'nsr';
