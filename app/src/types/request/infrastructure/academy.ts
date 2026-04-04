// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

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
