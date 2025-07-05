// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { clamp } from 'es-toolkit';
import { invoke } from '@tauri-apps/api/core';

export function cheatSetFood(food: number) {
  food = Math.trunc(Math.max(0, food));
  return invoke<null>('cheat_set_food', { food });
}

export function cheatSetIron(iron: number) {
  iron = Math.trunc(Math.max(0, iron));
  return invoke<null>('cheat_set_iron', { iron });
}

export function cheatSetMaxResources() {
  return invoke<null>('cheat_set_max_resources');
}

export function cheatSetResources(resources: Resources) {
  return invoke<null>('cheat_set_resources', { resources });
}

export function cheatSetStability(coord: Coord, stability: number) {
  stability = clamp(stability, 0.0, 1.0);
  return invoke<null>('cheat_set_stability', { coord, stability });
}

export function cheatSetStone(stone: number) {
  stone = Math.trunc(Math.max(0, stone));
  return invoke<null>('cheat_set_stone', { stone });
}

export function cheatSetWood(wood: number) {
  wood = Math.trunc(Math.max(0, wood));
  return invoke<null>('cheat_set_wood', { wood });
}
