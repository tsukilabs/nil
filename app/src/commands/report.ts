// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { toArray } from '@tb-dev/utils';
import { invoke } from '@tauri-apps/api/core';
import type { GetReportRequest, GetReportsRequest } from '@/lib/request';

export async function getReport(id: ReportId) {
  const req: GetReportRequest = {
    world: NIL.world.getIdStrict(),
    id,
  };

  return invoke<ReportKind>('get_report', { req });
}

export async function getReports(ids: MaybeReadonlyArray<ReportId>, limit?: Option<number>) {
  ids = toArray(ids as ReportId[]);
  limit ??= null;

  const req: GetReportsRequest = {
    world: NIL.world.getIdStrict(),
    ids,
    limit,
  };

  return invoke<readonly ReportKind[]>('get_reports', { req });
}
