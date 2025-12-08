// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface BattleResult {
  readonly attackerPersonnel: ArmyPersonnel;
  readonly attackerSurvivingPersonnel: ArmyPersonnel;
  readonly defenderPersonnel: ArmyPersonnel;
  readonly defenderSurvivingPersonnel: ArmyPersonnel;
  readonly wallLevel: BuildingLevel;
  readonly winner: BattleWinner;
}

type BattleWinner = 'attacker' | 'defender';
