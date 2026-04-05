// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { ArmyPersonnel } from '@/types/core/military/army';
import type { BuildingLevel, BuildingLevelDiff } from '@/types/core/infrastructure/building';

export interface BattleResult {
  readonly attackerPersonnel: ArmyPersonnel;
  readonly attackerSurvivingPersonnel: ArmyPersonnel;
  readonly defenderPersonnel: ArmyPersonnel;
  readonly defenderSurvivingPersonnel: ArmyPersonnel;
  readonly wallLevel: BuildingLevel;
  readonly downgradedWallLevel: BuildingLevelDiff;
  readonly winner: BattleWinner;
  readonly luck: Luck;
}

export type BattleWinner = 'attacker' | 'defender';

export type Luck = number;
