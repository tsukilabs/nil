// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { WorldId } from '@/types/core/world';
import type { Coord } from '@/types/core/continent';
import type { InfrastructureQueueOrderId } from '@/types/core/infrastructure/queue';
import type { StableRecruitOrderRequest } from '@/types/core/infrastructure/stable';

export interface AddStableRecruitOrderRequest {
  readonly world: WorldId;
  readonly request: StableRecruitOrderRequest;
}

export interface CancelStableRecruitOrderRequest {
  readonly world: WorldId;
  readonly coord: Coord;
  readonly id: InfrastructureQueueOrderId;
}

export interface GetStableRecruitCatalogRequest {
  readonly world: WorldId;
  readonly coord: Coord;
}
