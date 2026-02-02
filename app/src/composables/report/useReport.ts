// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { toRef, watch } from 'vue';
import { asyncRef } from '@tb-dev/vue';
import { getReport } from '@/commands/report';
import type { ReportImpl } from '@/core/model/report/abstract';
import { BattleReportImpl } from '@/core/model/report/battle-report';
import { SupportReportImpl } from '@/core/model/report/support-report';

export function useReport(id: MaybeNilRef<ReportId>) {
  const idRef = toRef(id);
  const { state, isLoading, execute } = asyncRef<Option<ReportImpl>>(null, async () => {
    return idRef.value ? getReport(idRef.value).then(toReportImpl) : null;
  });

  watch(idRef, execute);

  return {
    report: state,
    loading: isLoading,
    load: execute,
  };
}

async function toReportImpl({ kind, report }: ReportKind) {
  switch (kind) {
    case 'battle': {
      return BattleReportImpl.load(report);
    }
    case 'support': {
      return SupportReportImpl.load(report);
    }
  }
}
