// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { toU32 } from '@/lib/number';
import { invoke } from '@tauri-apps/api/core';
import { ResourcesImpl } from '@/core/model/resources';

export function cheatSetFood(food: number) {
  food = toU32(food);
  return invoke<null>('cheat_set_food', { food });
}

export function cheatSetIron(iron: number) {
  iron = toU32(iron);
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
  const impl = ResourcesImpl.create(resources);
  return invoke<null>('cheat_set_resources', { resources: impl });
}

export function cheatSetStone(stone: number) {
  stone = toU32(stone);
  return invoke<null>('cheat_set_stone', { stone });
}

export function cheatSetWood(wood: number) {
  wood = toU32(wood);
  return invoke<null>('cheat_set_wood', { wood });
}
