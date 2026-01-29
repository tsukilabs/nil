// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type GameScene =
  | InfrastructureScene
  | ProfileScene
  | ReportScene
  | WarRoomScene
  | 'chat'
  | 'city'
  | 'continent'
  | 'ranking';

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

type ProfileScene = 'profile-bot' | 'profile-city' | 'profile-player' | 'profile-precursor';

type ReportScene = 'report' | 'report-view';

type WarRoomScene = 'war-room' | 'war-room-simulator';
