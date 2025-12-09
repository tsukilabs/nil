// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';
import { asyncRef } from '@tb-dev/vue';
import type { ReportImpl } from '@/core/model/report/abstract';
import { BattleReportImpl } from '@/core/model/report/battle-report';

export function useReport(id: Option<ReportId>) {
  const { state, isLoading, execute } = asyncRef(null, async () => {
    return id ? toReportImpl(await commands.getReport(id)) : null;
  });

  return {
    reports: state,
    loading: isLoading,
    load: execute,
  };
}

export function toReportImpl({ kind, report }: ReportKind): ReportImpl {
  switch (kind) {
    // eslint-disable-next-line @typescript-eslint/no-unnecessary-condition
    case 'battle': {
      return BattleReportImpl.create(report);
    }
  }
}
