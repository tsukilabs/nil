// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Ref } from 'vue';
import * as commands from '@/commands';
import { asyncRef } from '@tb-dev/vue';
import { CoordImpl } from '@/core/model/continent/coord';
import type { ReportImpl } from '@/core/model/report/abstract';
import { BattleReportImpl } from '@/core/model/report/battle-report';
import { SupportReportImpl } from '@/core/model/report/support-report';

type CityCache = Map<ContinentIndex, MaybePromise<PublicCity>>;

export function useReports(ids: Ref<readonly ReportId[]>) {
  const { state, loading, load } = asyncRef<ReportImpl[]>([], async () => {
    if (ids.value.length > 0) {
      const reports = await commands.getReports(ids.value);
      const cityCache: CityCache = new Map();
      return Promise.all(reports.map((report) => {
        return toReportImpl(report, cityCache);
      }));
    }

    return [];
  });

  return {
    reports: state,
    loading,
    load,
  };
}

async function toReportImpl({ kind, report }: ReportKind, cityCache: CityCache) {
  switch (kind) {
    case 'battle': {
      return BattleReportImpl.create({
        report,
        originCity: await getCity(cityCache, report.origin),
        destinationCity: await getCity(cityCache, report.destination),
      });
    }
    case 'support': {
      return SupportReportImpl.create({
        report,
        originCity: await getCity(cityCache, report.origin),
        destinationCity: await getCity(cityCache, report.destination),
      });
    }
  }
}

async function getCity(cache: CityCache, coord: Coord) {
  const index = CoordImpl.toContinentIndex(coord);
  let city = cache.get(index);
  if (!city) {
    city = commands.getPublicCity(coord);
    cache.set(index, city);
  }

  return city;
}
