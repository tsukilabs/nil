// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { WorldId } from '@/types/core/world';
import type { PrecursorId } from '@/types/core/npc/precursor';

export interface GetPrecursorCoordsRequest {
  readonly world: WorldId;
  readonly id: PrecursorId;
}

export interface GetPublicPrecursorRequest {
  readonly world: WorldId;
  readonly id: PrecursorId;
}

export interface GetPublicPrecursorsRequest {
  readonly world: WorldId;
}
