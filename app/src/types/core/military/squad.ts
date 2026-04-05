// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { UnitId } from '@/types/core/military/unit';

export interface Squad {
  readonly unit: UnitId;
  readonly size: number;
}

export type SquadTuple = [Squad['unit'], Squad['size']];
