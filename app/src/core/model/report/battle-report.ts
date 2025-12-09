// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { ResourcesImpl } from '@/core/model/resources';
import { CoordImpl } from '@/core/model/continent/coord';
import { ReportImpl } from '@/core/model/report/abstract';
import { BattleResultImpl } from '@/core/model/battle-result';

export class BattleReportImpl extends ReportImpl implements BattleReport {
  public readonly attacker: Ruler;
  public readonly defender: Ruler;
  public readonly result: BattleResultImpl;
  public readonly city: PublicCity;
  public readonly hauledResources: ResourcesImpl;

  private constructor(report: BattleReport) {
    super(report);
    this.attacker = report.attacker;
    this.defender = report.defender;
    this.result = BattleResultImpl.create(report.result);
    this.city = report.city;
    this.hauledResources = ResourcesImpl.create(report.hauledResources);
  }

  public override getTitle() {
    return super.i18n((t) => {
      return t('battle-report-title', {
        attacker: this.attacker.id,
        targetCityName: this.city.name,
        targetCoord: CoordImpl.format(this.city.coord),
      });
    });
  }

  public static create(report: BattleReport) {
    if (report instanceof BattleReportImpl) {
      return report;
    }

    return new BattleReportImpl(report);
  }
}
