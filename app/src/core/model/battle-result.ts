// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { toI8 } from '@/lib/number';
import { simulateBattle } from '@/commands/battle';
import { ArmyPersonnelImpl } from '@/core/model/military/army-personnel';

export class BattleResultImpl implements BattleResult {
  public readonly winner: BattleWinner;
  public readonly attackerPersonnel: ArmyPersonnelImpl;
  public readonly attackerSurvivingPersonnel: ArmyPersonnelImpl;
  public readonly defenderPersonnel: ArmyPersonnelImpl;
  public readonly defenderSurvivingPersonnel: ArmyPersonnelImpl;
  public readonly wallLevel: BuildingLevel;
  public readonly downgradedWallLevel: BuildingLevel;
  public readonly luck: Luck;

  private constructor(result: BattleResult) {
    this.winner = result.winner;
    this.attackerPersonnel = ArmyPersonnelImpl.create(result.attackerPersonnel);
    this.attackerSurvivingPersonnel = ArmyPersonnelImpl.create(result.attackerSurvivingPersonnel);
    this.defenderPersonnel = ArmyPersonnelImpl.create(result.defenderPersonnel);
    this.defenderSurvivingPersonnel = ArmyPersonnelImpl.create(result.defenderSurvivingPersonnel);
    this.wallLevel = result.wallLevel;
    this.downgradedWallLevel = result.downgradedWallLevel;
    this.luck = result.luck;
  }

  public getAttackerLosses() {
    return BattleResultImpl.getLosses(
      this.attackerPersonnel,
      this.attackerSurvivingPersonnel,
    );
  }

  public getDefenderLosses() {
    return BattleResultImpl.getLosses(
      this.defenderPersonnel,
      this.defenderSurvivingPersonnel,
    );
  }

  public formatLuck() {
    return BattleResultImpl.formatLuck(this.luck);
  }

  public static create(result: BattleResult) {
    if (result instanceof BattleResultImpl) {
      return result;
    }

    return new BattleResultImpl(result);
  }

  public static async simulate(args: Parameters<typeof simulateBattle>[0]) {
    return new BattleResultImpl(await simulateBattle(args));
  }

  public static getLosses(personnel: ArmyPersonnel, surviving: ArmyPersonnel) {
    return ArmyPersonnelImpl.sub(personnel, surviving);
  }

  public static formatLuck(luck: Luck) {
    return `${toI8(luck)}%`;
  }
}
