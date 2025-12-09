// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { getPublicCity } from '@/commands/city';
import type { ComposerTranslation } from 'vue-i18n';
import { ResourcesImpl } from '@/core/model/resources';
import { CoordImpl } from '@/core/model/continent/coord';
import { ReportImpl } from '@/core/model/report/abstract';
import enUS from '@/locale/en-US/scenes/game/report.json';
import { BattleResultImpl } from '@/core/model/battle-result';

export class BattleReportImpl extends ReportImpl implements BattleReport {
  public readonly attacker: Ruler;
  public readonly defender: Ruler;
  public readonly origin: CoordImpl;
  public readonly destination: CoordImpl;
  public readonly result: BattleResultImpl;
  public readonly hauledResources: ResourcesImpl;

  public readonly originCity: PublicCity;
  public readonly destinationCity: PublicCity;

  private constructor(args: BattleReportImplConstructorArgs) {
    super(args.report);
    this.attacker = args.report.attacker;
    this.defender = args.report.defender;
    this.origin = CoordImpl.create(args.report.origin);
    this.destination = CoordImpl.create(args.report.destination);
    this.result = BattleResultImpl.create(args.report.result);
    this.hauledResources = ResourcesImpl.create(args.report.hauledResources);

    this.originCity = args.originCity;
    this.destinationCity = args.destinationCity;
  }

  public override getTitle(t: ComposerTranslation<typeof enUS>) {
    return t('battle-report-title', {
      attacker: this.attacker.id,
      destination: this.destination.format(),
      destinationName: this.destinationCity.name,
    });
  }

  public static create(args: BattleReportImplConstructorArgs) {
    if (args.report instanceof BattleReportImpl) {
      return args.report;
    }

    return new BattleReportImpl(args);
  }

  public static async load(report: BattleReport) {
    const [originCity, destinationCity] = await Promise.all([
      getPublicCity(report.origin),
      getPublicCity(report.destination),
    ]);

    return BattleReportImpl.create({
      report,
      originCity,
      destinationCity,
    });
  }
}

export interface BattleReportImplConstructorArgs {
  report: BattleReport;
  originCity: PublicCity;
  destinationCity: PublicCity;
}
