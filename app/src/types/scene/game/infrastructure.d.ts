// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type InfrastructureScene =
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

type AcademyScene = 'academy' | 'academy-settings';

type PrefectureScene = 'prefecture' | 'prefecture-settings';

type StableScene = 'stable' | 'stable-settings';

type WorkshopScene = 'workshop' | 'workshop-settings';
