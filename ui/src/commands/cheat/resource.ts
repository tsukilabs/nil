// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export function cheatSetFood(food: number) {
  food = Math.trunc(Math.max(0, food));
  return invoke<null>('cheat_set_food', { food });
}

export function cheatSetIron(iron: number) {
  iron = Math.trunc(Math.max(0, iron));
  return invoke<null>('cheat_set_iron', { iron });
}

export function cheatSetMaxFood() {
  return invoke<null>('cheat_set_max_food');
}

export function cheatSetMaxIron() {
  return invoke<null>('cheat_set_max_iron');
}

export function cheatSetMaxResources() {
  return invoke<null>('cheat_set_max_resources');
}

export function cheatSetMaxSiloResources() {
  return invoke<null>('cheat_set_max_silo_resources');
}

export function cheatSetMaxStone() {
  return invoke<null>('cheat_set_max_stone');
}

export function cheatSetMaxWarehouseResources() {
  return invoke<null>('cheat_set_max_warehouse_resources');
}

export function cheatSetMaxWood() {
  return invoke<null>('cheat_set_max_wood');
}

export function cheatSetResources(resources: Resources) {
  return invoke<null>('cheat_set_resources', { resources });
}

export function cheatSetStone(stone: number) {
  stone = Math.trunc(Math.max(0, stone));
  return invoke<null>('cheat_set_stone', { stone });
}

export function cheatSetWood(wood: number) {
  wood = Math.trunc(Math.max(0, wood));
  return invoke<null>('cheat_set_wood', { wood });
}
