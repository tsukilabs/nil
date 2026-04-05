// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export type Resource = 'food' | 'iron' | 'stone' | 'wood';

export interface Resources {
  readonly food: number;
  readonly iron: number;
  readonly stone: number;
  readonly wood: number;
}
