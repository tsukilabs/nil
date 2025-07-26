// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type Scene = 'home' | 'host-game' | 'join-game' | 'settings' | GameScene;

type GameScene = 'continent' | 'village' | InfrastructureScene | ScriptScene;

type InfrastructureScene =
  | AcademyScene
  | PrefectureScene
  | StableScene
  | 'farm'
  | 'iron-mine'
  | 'quarry'
  | 'sawmill'
  | 'silo'
  | 'wall'
  | 'warehouse';

type AcademyScene = 'academy' | 'academy-settings';
type PrefectureScene = 'prefecture' | 'prefecture-settings';
type StableScene = 'stable' | 'stable-settings';

type ScriptScene = 'script' | 'nsr';
