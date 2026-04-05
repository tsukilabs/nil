// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { WorldId } from '@/types/core/world';
import type { ManeuverRequest } from '@/types/core/military/maneuver';

export interface RequestManeuverRequest {
  readonly world: WorldId;
  readonly request: ManeuverRequest;
}
