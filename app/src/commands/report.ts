// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { toArray } from '@tb-dev/utils';
import { invoke } from '@tauri-apps/api/core';

export async function getReport(id: ReportId) {
  return invoke<ReportKind>('get_report', { req: { id } });
}

export async function getReports(ids: MaybeArray<ReportId>, limit?: Option<number>) {
  ids = toArray(ids);
  limit ??= null;
  return invoke<readonly ReportKind[]>('get_reports', { req: { ids, limit } });
}
