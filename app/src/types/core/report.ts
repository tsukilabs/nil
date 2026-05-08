// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { ReportId, RoundId } from '@/types/bindings';

export interface Report_ {
  readonly id: ReportId;
  readonly round: RoundId;
  readonly time: string;
}
