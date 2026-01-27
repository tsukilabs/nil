// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export type * from './stable';
export type * from './academy';
export type * from './prefecture';

export interface ToggleBuildingRequest {
  readonly world: WorldId;
  readonly coord: Coord;
  readonly id: BuildingId;
  readonly enabled: boolean;
}
