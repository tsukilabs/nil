// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

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
