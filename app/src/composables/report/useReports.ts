// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Ref } from 'vue';
import * as commands from '@/commands';
import { asyncRef } from '@tb-dev/vue';
import { toReportImpl } from '@/composables/report/useReport';

export function useReports(ids: Ref<readonly ReportId[]>) {
  const { state, isLoading, execute } = asyncRef([], async () => {
    const reports = await commands.getReports(ids.value);
    return reports.map(toReportImpl);
  });

  return {
    reports: state,
    loading: isLoading,
    load: execute,
  };
}
