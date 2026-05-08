// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import { type MaybeArray, type Option, toArray } from '@tb-dev/utils';
import type {
  ForwardReportRequest,
  GetReportRequest,
  GetReportResponse,
  GetReportsRequest,
  GetReportsResponse,
  PlayerId,
  RemoveReportRequest,
  ReportId,
} from '@tsukilabs/nil-bindings';

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

  return invoke<GetReportResponse>('get_report', { req });
}

export async function getReports(ids: MaybeArray<ReportId>, limit?: Option<number>) {
  ids = toArray(ids as ReportId[]);
  limit ??= null;

  const req: GetReportsRequest = {
    world: NIL.world.getIdStrict(),
    ids,
    limit,
  };

  return invoke<GetReportsResponse>('get_reports', { req });
}

export async function removeReport(id: ReportId) {
  const req: RemoveReportRequest = {
    world: NIL.world.getIdStrict(),
    id,
  };

  await invoke('remove_report', { req });
}
