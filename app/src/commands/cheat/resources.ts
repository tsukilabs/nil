// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { toU32 } from '@/lib/number';
import { invoke } from '@tauri-apps/api/core';
import { getCityOwner } from '@/commands/city';
import { ResourcesImpl } from '@/core/model/resources';

export async function cheatGetResources(ruler?: Option<Ruler>) {
  return invoke<Resources>('cheat_get_resources', { req: { ruler } });
}

export async function cheatGetBotResources(bot: BotId) {
  return cheatGetResources({ kind: 'bot', id: bot });
}

export async function cheatGetOwnerResources(coord: ContinentKey) {
  return cheatGetResources(await getCityOwner(coord));
}

export async function cheatGetPlayerResources(player: PlayerId) {
  return cheatGetResources({ kind: 'player', id: player });
}

export async function cheatGetPrecursorResources(precursor: PrecursorId) {
  return cheatGetResources({ kind: 'precursor', id: precursor });
}

export async function cheatSetFood(food: number, ruler?: Option<Ruler>) {
  food = toU32(food);
  await invoke('cheat_set_food', { req: { food, ruler } });
}

export async function cheatSetIron(iron: number, ruler?: Option<Ruler>) {
  iron = toU32(iron);
  await invoke('cheat_set_iron', { req: { iron, ruler } });
}

export async function cheatSetMaxFood(ruler?: Option<Ruler>) {
  await invoke('cheat_set_max_food', { req: { ruler } });
}

export async function cheatSetMaxIron(ruler?: Option<Ruler>) {
  await invoke('cheat_set_max_iron', { req: { ruler } });
}

export async function cheatSetMaxResources(ruler?: Option<Ruler>) {
  await invoke('cheat_set_max_resources', { req: { ruler } });
}

export async function cheatSetMaxSiloResources(ruler?: Option<Ruler>) {
  await invoke('cheat_set_max_silo_resources', { req: { ruler } });
}

export async function cheatSetMaxStone(ruler?: Option<Ruler>) {
  await invoke('cheat_set_max_stone', { req: { ruler } });
}

export async function cheatSetMaxWarehouseResources(ruler?: Option<Ruler>) {
  await invoke('cheat_set_max_warehouse_resources', { req: { ruler } });
}

export async function cheatSetMaxWood(ruler?: Option<Ruler>) {
  await invoke('cheat_set_max_wood', { req: { ruler } });
}

export async function cheatSetOwnerFood(coord: ContinentKey, food: number) {
  return cheatSetFood(food, await getCityOwner(coord));
}

export async function cheatSetOwnerIron(coord: ContinentKey, iron: number) {
  return cheatSetIron(iron, await getCityOwner(coord));
}

export async function cheatSetOwnerMaxFood(coord: ContinentKey) {
  return cheatSetMaxFood(await getCityOwner(coord));
}

export async function cheatSetOwnerMaxIron(coord: ContinentKey) {
  return cheatSetMaxIron(await getCityOwner(coord));
}

export async function cheatSetOwnerMaxResources(coord: ContinentKey) {
  return cheatSetMaxResources(await getCityOwner(coord));
}

export async function cheatSetOwnerMaxSiloResources(coord: ContinentKey) {
  return cheatSetMaxSiloResources(await getCityOwner(coord));
}

export async function cheatSetOwnerMaxStone(coord: ContinentKey) {
  return cheatSetMaxStone(await getCityOwner(coord));
}

export async function cheatSetOwnerMaxWarehouseResources(coord: ContinentKey) {
  return cheatSetMaxWarehouseResources(await getCityOwner(coord));
}

export async function cheatSetOwnerMaxWood(coord: ContinentKey) {
  return cheatSetMaxWood(await getCityOwner(coord));
}

export async function cheatSetOwnerResources(coord: ContinentKey, resources: Resources) {
  return cheatSetResources(resources, await getCityOwner(coord));
}

export async function cheatSetOwnerStone(coord: ContinentKey, stone: number) {
  return cheatSetStone(stone, await getCityOwner(coord));
}

export async function cheatSetOwnerWood(coord: ContinentKey, wood: number) {
  return cheatSetWood(wood, await getCityOwner(coord));
}

export async function cheatSetResources(resources: Resources, ruler?: Option<Ruler>) {
  resources = ResourcesImpl.create(resources);
  await invoke('cheat_set_resources', { req: { resources, ruler } });
}

export async function cheatSetStone(stone: number, ruler?: Option<Ruler>) {
  stone = toU32(stone);
  await invoke('cheat_set_stone', { req: { stone, ruler } });
}

export async function cheatSetWood(wood: number, ruler?: Option<Ruler>) {
  wood = toU32(wood);
  await invoke('cheat_set_wood', { req: { wood, ruler } });
}
