// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { InfrastructureScene } from '@/types/scene/game/infrastructure';

export type GameScene =
  | ContinentScene
  | InfrastructureScene
  | ProfileScene
  | ReportScene
  | WarRoomScene
  | 'chat'
  | 'city'
  | 'continent'
  | 'ranking'
  | 'scripts';

export type ContinentScene = 'continent' | 'continent-cities';

export type ProfileScene = 'profile-bot' | 'profile-city' | 'profile-player' | 'profile-precursor';

export type ReportScene = 'report' | 'report-forward' | 'report-view';

export type WarRoomScene = 'war-room' | 'war-room-simulator';
