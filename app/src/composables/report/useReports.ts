// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { watch } from "vue";
import * as commands from "@/commands";
import { asyncRef } from "@tb-dev/vue";
import type { MaybePromise } from "@tb-dev/utils";
import { CoordImpl } from "@/core/model/continent/coord";
import { compareDesc as compareDateDesc } from "date-fns";
import type { ReportImpl } from "@/core/model/report/abstract";
import { BattleReportImpl } from "@/core/model/report/battle-report";
import { SupportReportImpl } from "@/core/model/report/support-report";
import type { ContinentIndex, Coord, PublicCity, ReportKind } from "@tsukilabs/nil-bindings";

type CityCache = Map<ContinentIndex, MaybePromise<PublicCity>>;

export function useReports() {
  const { reports } = NIL.report.refs();
  const { state, loading, load } = asyncRef<ReportImpl[]>([], async () => {
    if (reports.value.length > 0) {
      const cityCache: CityCache = new Map();
      const impl = await Promise.all(reports.value.map((report) => {
        return toReportImpl(report, cityCache);
      }));

      return impl.toSorted((a, b) => {
        return compareDateDesc(a.date, b.date);
      });
    }
    else {
      return [];
    }
  });

  watch(reports, () => void load());

  return {
    reports: state,
    loading,
    loadReports: load,
  };
}

async function toReportImpl({ kind, report }: ReportKind, cityCache: CityCache) {
  switch (kind) {
    case "battle": {
      return BattleReportImpl.create({
        report,
        originCity: await getCity(cityCache, report.origin),
        destinationCity: await getCity(cityCache, report.destination),
      });
    }
    case "support": {
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
    const promise = commands.getPublicCity({ coord });
    city = promise.then((it) => it.city);
    cache.set(index, city);
  }

  return city;
}
