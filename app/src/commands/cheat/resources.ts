// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { toU32 } from '@/lib/number';
import { invoke } from '@tauri-apps/api/core';
import { ResourcesImpl } from '@/core/model/resources';

export async function cheatGetResources(ruler: Ruler) {
  return invoke<Resources>('cheat_get_resources', { req: { ruler } });
}

export async function cheatGetBotResources(bot: BotId) {
  return cheatGetResources({ kind: 'bot', id: bot });
}

export async function cheatGetPlayerResources(player: PlayerId) {
  return cheatGetResources({ kind: 'player', id: player });
}

export async function cheatGetPrecursorResources(precursor: PrecursorId) {
  return cheatGetResources({ kind: 'precursor', id: precursor });
}

export async function cheatSetFood(food: number) {
  food = toU32(food);
  await invoke('cheat_set_food', { req: { food } });
}

export async function cheatSetIron(iron: number) {
  iron = toU32(iron);
  await invoke('cheat_set_iron', { req: { iron } });
}

export async function cheatSetMaxFood() {
  await invoke('cheat_set_max_food');
}

export async function cheatSetMaxIron() {
  await invoke('cheat_set_max_iron');
}

export async function cheatSetMaxResources() {
  await invoke('cheat_set_max_resources');
}

export async function cheatSetMaxSiloResources() {
  await invoke('cheat_set_max_silo_resources');
}

export async function cheatSetMaxStone() {
  await invoke('cheat_set_max_stone');
}

export async function cheatSetMaxWarehouseResources() {
  await invoke('cheat_set_max_warehouse_resources');
}

export async function cheatSetMaxWood() {
  await invoke('cheat_set_max_wood');
}

export async function cheatSetResources(resources: Resources) {
  const impl = ResourcesImpl.create(resources);
  await invoke('cheat_set_resources', { req: { resources: impl } });
}

export async function cheatSetStone(stone: number) {
  stone = toU32(stone);
  await invoke('cheat_set_stone', { req: { stone } });
}

export async function cheatSetWood(wood: number) {
  wood = toU32(wood);
  await invoke('cheat_set_wood', { req: { wood } });
}
