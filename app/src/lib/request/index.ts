// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export type * from './npc';
export type * from './chat';
export type * from './city';
export type * from './cheat';
export type * from './round';
export type * from './world';
export type * from './battle';
export type * from './player';
export type * from './report';
export type * from './ranking';
export type * from './military';
export type * from './continent';
export type * from './infrastructure';

export interface LeaveRequest {
  readonly world: WorldId;
}
