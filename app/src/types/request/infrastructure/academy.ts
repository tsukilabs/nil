// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { WorldId } from '@/types/core/world';
import type { Coord } from '@/types/core/continent';
import type { InfrastructureQueueOrderId } from '@/types/core/infrastructure/queue';
import type { AcademyRecruitOrderRequest } from '@/types/core/infrastructure/academy';

export interface AddAcademyRecruitOrderRequest {
  readonly world: WorldId;
  readonly request: AcademyRecruitOrderRequest;
}

export interface CancelAcademyRecruitOrderRequest {
  readonly world: WorldId;
  readonly coord: Coord;
  readonly id: InfrastructureQueueOrderId;
}

export interface GetAcademyRecruitCatalogRequest {
  readonly world: WorldId;
  readonly coord: Coord;
}
