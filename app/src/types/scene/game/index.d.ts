// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type GameScene =
  | ContinentScene
  | InfrastructureScene
  | ProfileScene
  | ReportScene
  | ScriptsScene
  | WarRoomScene
  | 'chat'
  | 'city'
  | 'continent'
  | 'ranking';

type ContinentScene = 'continent' | 'continent-cities';

type ProfileScene = 'profile-bot' | 'profile-city' | 'profile-player' | 'profile-precursor';

type ReportScene = 'report' | 'report-view';

type ScriptsScene = 'scripts' | 'scripts-terminal';

type WarRoomScene = 'war-room' | 'war-room-simulator';
