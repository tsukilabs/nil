// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { toArray } from '@tb-dev/utils';
import { invoke } from '@tauri-apps/api/core';
import type { PlayerId } from '@/types/core/player';
import type { ReportId, ReportKind } from '@/types/core/report';
import type {
  ForwardReportRequest,
  GetReportRequest,
  GetReportsRequest,
  RemoveReportRequest,
} from '@/types/request/report';

export async function forwardReport(id: ReportId, recipient: PlayerId) {
  const req: ForwardReportRequest = {
    world: NIL.world.getIdStrict(),
    id,
    recipient,
  };

  await invoke('forward_report', { req });
}

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

export async function removeReport(id: ReportId) {
  const req: RemoveReportRequest = {
    world: NIL.world.getIdStrict(),
    id,
  };

  await invoke('remove_report', { req });
}
