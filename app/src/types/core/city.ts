// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Ruler } from '@/types/core/ruler';
import type { Coord } from '@/types/core/continent';

export interface PublicCity {
  readonly coord: Coord;
  readonly name: string;
  readonly owner: Ruler;
}

export interface City extends PublicCity {
  readonly infrastructure: Infrastructure;
  readonly stability: number;
}
