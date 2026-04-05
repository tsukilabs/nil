// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { WorldId } from '@/types/core/world';
import type { PlayerId } from '@/types/core/player';
import type { ReportId } from '@/types/core/report';

export interface ForwardReportRequest {
  readonly world: WorldId;
  readonly id: ReportId;
  readonly recipient: PlayerId;
}

export interface GetReportRequest {
  readonly world: WorldId;
  readonly id: ReportId;
}

export interface GetReportsRequest {
  readonly world: WorldId;
  readonly ids: readonly ReportId[];
  readonly limit: Option<number>;
}
