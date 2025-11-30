// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { clamp } from 'es-toolkit';
import { invoke } from '@tauri-apps/api/core';

export function cheatSetStability(coord: Coord, stability: number) {
  stability = clamp(stability, 0.0, 1.0);
  return invoke<null>('cheat_set_stability', { coord, stability });
}
