// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { toRef, watch } from "vue";
import type { Option } from "@tb-dev/utils";
import { asyncRef, type MaybeNilRef } from "@tb-dev/vue";
import type { ReportImpl } from "@/core/model/report/abstract";
import type { ReportId, ReportKind } from "@tsukilabs/nil-bindings";
import { BattleReportImpl } from "@/core/model/report/battle-report";
import { SupportReportImpl } from "@/core/model/report/support-report";

export function useReport(id: MaybeNilRef<ReportId>) {
  const idRef = toRef(id);
  const { state, loading, load } = asyncRef<Option<ReportImpl>>(null, async () => {
    if (idRef.value) {
      const report = NIL.report.getReport(idRef.value);
      return report ? toReportImpl(report) : null;
    }
    else {
      return null;
    }
  });

  watch(idRef, load);

  return {
    report: state,
    loading,
    load,
  };
}

async function toReportImpl({ kind, report }: ReportKind) {
  switch (kind) {
    case "battle": {
      return BattleReportImpl.load(report);
    }
    case "support": {
      return SupportReportImpl.load(report);
    }
  }
}
