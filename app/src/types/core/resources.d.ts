// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type Resource = 'food' | 'iron' | 'stone' | 'wood';

interface Resources {
  readonly food: number;
  readonly iron: number;
  readonly stone: number;
  readonly wood: number;
}
