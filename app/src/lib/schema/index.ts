// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { worldOptions } from './world';
import { playerOptions } from './player';

export function isPlayerOptions(value: unknown): value is PlayerOptions {
  return playerOptions.safeParse(value).success;
}

export function isWorldOptions(value: unknown): value is WorldOptions {
  return worldOptions.safeParse(value).success;
}
