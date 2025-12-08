// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { ResourcesImpl } from '@/core/model/resources';
import { ReportImpl } from '@/core/model/report/abstract';
import { BattleResultImpl } from '@/core/model/battle-result';

export class BattleReportImpl extends ReportImpl implements BattleReport {
  public readonly attacker: Ruler;
  public readonly defender: Ruler;
  public readonly hauledResources: ResourcesImpl;
  public readonly result: BattleResultImpl;

  private constructor(report: BattleReport) {
    super(report);
    this.attacker = report.attacker;
    this.defender = report.defender;
    this.hauledResources = ResourcesImpl.create(report.hauledResources);
    this.result = BattleResultImpl.create(report.result);
  }

  public static create(report: BattleReport) {
    if (report instanceof BattleReportImpl) {
      return report;
    }

    return new BattleReportImpl(report);
  }
}
