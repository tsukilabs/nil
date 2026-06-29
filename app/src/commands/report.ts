// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from "@tauri-apps/api/core";
import type { ForwardReportRequest, PlayerId, ReportKind } from "@tsukilabs/nil-bindings";

export async function forwardReport(recipient: PlayerId, report: ReportKind) {
  const req: ForwardReportRequest = {
    world: NIL.world.getIdStrict(),
    recipient,
    report,
  };

  await invoke("forward_report", { req });
}
