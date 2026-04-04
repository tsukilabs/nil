// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

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
