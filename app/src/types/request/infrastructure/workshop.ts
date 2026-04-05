// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { WorldId } from '@/types/core/world';
import type { Coord } from '@/types/core/continent';
import type { InfrastructureQueueOrderId } from '@/types/core/infrastructure/queue';
import type { WorkshopRecruitOrderRequest } from '@/types/core/infrastructure/workshop';

export interface AddWorkshopRecruitOrderRequest {
  readonly world: WorldId;
  readonly request: WorkshopRecruitOrderRequest;
}

export interface CancelWorkshopRecruitOrderRequest {
  readonly world: WorldId;
  readonly coord: Coord;
  readonly id: InfrastructureQueueOrderId;
}

export interface GetWorkshopRecruitCatalogRequest {
  readonly world: WorldId;
  readonly coord: Coord;
}
