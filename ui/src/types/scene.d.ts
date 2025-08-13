// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type Scene = GameScene | 'home' | 'host-game' | 'join-game' | 'load-game' | 'settings';

type GameScene = InfrastructureScene | ProfileScene | ScriptScene | 'continent' | 'village';

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

type ProfileScene = 'profile-village';

type ScriptScene = 'script' | 'nsr';
