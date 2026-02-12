// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { getPublicCities } from '@/commands/city';
import type { ComposerTranslation } from 'vue-i18n';
import { CoordImpl } from '@/core/model/continent/coord';
import { ReportImpl } from '@/core/model/report/abstract';
import enUS from '@/locale/en-US/scenes/game/report.json';
import { ArmyPersonnelImpl } from '@/core/model/military/army-personnel';

export class SupportReportImpl extends ReportImpl implements SupportReport {
  public readonly sender: Ruler;
  public readonly receiver: Ruler;
  public readonly origin: CoordImpl;
  public readonly destination: CoordImpl;
  public readonly personnel: ArmyPersonnelImpl;

  public readonly originCity: PublicCity;
  public readonly destinationCity: PublicCity;

  private constructor(args: SupportReportImplConstructorArgs) {
    super(args.report);
    this.sender = args.report.sender;
    this.receiver = args.report.receiver;
    this.origin = CoordImpl.create(args.report.origin);
    this.destination = CoordImpl.create(args.report.destination);
    this.personnel = ArmyPersonnelImpl.create(args.report.personnel);

    this.originCity = args.originCity;
    this.destinationCity = args.destinationCity;
  }

  public override getTitle(t: ComposerTranslation<typeof enUS>) {
    return t('support-report-title', {
      sender: this.sender.id,
      destination: this.destination.format(),
      destinationName: this.destinationCity.name,
    });
  }

  public static create(args: SupportReportImplConstructorArgs) {
    if (args.report instanceof SupportReportImpl) {
      return args.report;
    }

    return new SupportReportImpl(args);
  }

  public static async load(report: SupportReport) {
    const response = await getPublicCities({
      coords: [report.origin, report.destination],
    });

    const eq = (a: Coord, b: Coord) => CoordImpl.isEqual(a, b);
    const originCity = response.find((it) => eq(it.city.coord, report.origin))!;
    const destinationCity = response.find((it) => eq(it.city.coord, report.destination))!;

    return SupportReportImpl.create({
      report,
      originCity: originCity.city,
      destinationCity: destinationCity.city,
    });
  }
}

export interface SupportReportImplConstructorArgs {
  report: SupportReport;
  originCity: PublicCity;
  destinationCity: PublicCity;
}
