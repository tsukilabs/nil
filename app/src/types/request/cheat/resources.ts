// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export interface CheatGetResourcesRequest {
  readonly world: WorldId;
  readonly ruler: Option<Ruler>;
}

export interface CheatSetFoodRequest {
  readonly world: WorldId;
  readonly ruler: Option<Ruler>;
  readonly food: number;
}

export interface CheatSetIronRequest {
  readonly world: WorldId;
  readonly ruler: Option<Ruler>;
  readonly iron: number;
}

export interface CheatSetMaxFoodRequest {
  readonly world: WorldId;
  readonly ruler: Option<Ruler>;
}

export interface CheatSetMaxIronRequest {
  readonly world: WorldId;
  readonly ruler: Option<Ruler>;
}

export interface CheatSetMaxResourcesRequest {
  readonly world: WorldId;
  readonly ruler: Option<Ruler>;
}

export interface CheatSetMaxSiloResourcesRequest {
  readonly world: WorldId;
  readonly ruler: Option<Ruler>;
}

export interface CheatSetMaxStoneRequest {
  readonly world: WorldId;
  readonly ruler: Option<Ruler>;
}

export interface CheatSetMaxWarehouseResourcesRequest {
  readonly world: WorldId;
  readonly ruler: Option<Ruler>;
}

export interface CheatSetMaxWoodRequest {
  readonly world: WorldId;
  readonly ruler: Option<Ruler>;
}

export interface CheatSetResourcesRequest {
  readonly world: WorldId;
  readonly ruler: Option<Ruler>;
  readonly resources: Resources;
}

export interface CheatSetStoneRequest {
  readonly world: WorldId;
  readonly ruler: Option<Ruler>;
  readonly stone: number;
}

export interface CheatSetWoodRequest {
  readonly world: WorldId;
  readonly ruler: Option<Ruler>;
  readonly wood: number;
}
