// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { password } from './utils';
import { worldOptions } from './world';
import { playerId, playerOptions } from './player';

export function isValidPlayerId(value: unknown): value is string {
  return playerId.safeParse(value).success;
}

export function isPlayerOptions(value: unknown): value is PlayerOptions {
  return playerOptions.safeParse(value).success;
}

export function isValidPassword(value: unknown): value is string {
  return password.safeParse(value).success;
}

export function isWorldOptions(value: unknown): value is WorldOptions {
  return worldOptions.safeParse(value).success;
}
