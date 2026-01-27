// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export interface GetReportRequest {
  readonly world: WorldId;
  readonly id: ReportId;
}

export interface GetReportsRequest {
  readonly world: WorldId;
  readonly ids: readonly ReportId[];
  readonly limit: Option<number>;
}
