// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Coord } from '@/types/core/continent';

export interface PublicPrecursor {
  readonly id: PrecursorId;
  readonly origin: Coord;
}

export type PrecursorId = 'A' | 'B';
