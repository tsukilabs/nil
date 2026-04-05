// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { BuildingId, BuildingLevel } from '@/types/core/infrastructure/building';

export interface BuildStep {
  readonly id: BuildingId;
  readonly level: BuildingLevel;
}
