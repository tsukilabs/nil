// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as options from './options';

export function isPlayerOptions(value: unknown): value is PlayerOptions {
  return options.player.safeParse(value).success;
}

export function isWorldOptions(value: unknown): value is WorldOptions {
  return options.world.safeParse(value).success;
}
