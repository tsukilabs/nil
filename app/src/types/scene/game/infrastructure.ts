// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export type InfrastructureScene =
  | AcademyScene
  | PrefectureScene
  | StableScene
  | WorkshopScene
  | 'farm'
  | 'iron-mine'
  | 'quarry'
  | 'sawmill'
  | 'silo'
  | 'wall'
  | 'warehouse';

export type AcademyScene = 'academy' | 'academy-settings';

export type PrefectureScene = 'prefecture' | 'prefecture-settings';

export type StableScene = 'stable' | 'stable-settings';

export type WorkshopScene = 'workshop' | 'workshop-settings';
