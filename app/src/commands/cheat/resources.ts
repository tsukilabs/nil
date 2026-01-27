// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { toU32 } from '@/lib/number';
import { invoke } from '@tauri-apps/api/core';
import { getCityOwner } from '@/commands/city';
import { ResourcesImpl } from '@/core/model/resources';
import type {
  CheatGetResourcesRequest,
  CheatSetFoodRequest,
  CheatSetIronRequest,
  CheatSetMaxFoodRequest,
  CheatSetMaxIronRequest,
  CheatSetMaxResourcesRequest,
  CheatSetMaxSiloResourcesRequest,
  CheatSetMaxStoneRequest,
  CheatSetMaxWarehouseResourcesRequest,
  CheatSetMaxWoodRequest,
  CheatSetResourcesRequest,
  CheatSetStoneRequest,
  CheatSetWoodRequest,
} from '@/lib/request';

export async function cheatGetResources(ruler?: Option<Ruler>) {
  const req: CheatGetResourcesRequest = {
    world: NIL.world.getIdStrict(),
    ruler: ruler ?? null,
  };

  return invoke<Resources>('cheat_get_resources', { req });
}

export async function cheatGetBotResources(bot: BotId) {
  return cheatGetResources({ kind: 'bot', id: bot });
}

export async function cheatGetOwnerResources(coord: ContinentKey) {
  const ruler = await getCityOwner(coord);
  return cheatGetResources(ruler);
}

export async function cheatGetPlayerResources(player: PlayerId) {
  return cheatGetResources({ kind: 'player', id: player });
}

export async function cheatGetPrecursorResources(precursor: PrecursorId) {
  return cheatGetResources({ kind: 'precursor', id: precursor });
}

export async function cheatSetFood(food: number, ruler?: Option<Ruler>) {
  food = toU32(food);
  const req: CheatSetFoodRequest = {
    world: NIL.world.getIdStrict(),
    food,
    ruler: ruler ?? null,
  };

  await invoke('cheat_set_food', { req });
}

export async function cheatSetIron(iron: number, ruler?: Option<Ruler>) {
  iron = toU32(iron);
  const req: CheatSetIronRequest = {
    world: NIL.world.getIdStrict(),
    iron,
    ruler: ruler ?? null,
  };

  await invoke('cheat_set_iron', { req });
}

export async function cheatSetMaxFood(ruler?: Option<Ruler>) {
  const req: CheatSetMaxFoodRequest = {
    world: NIL.world.getIdStrict(),
    ruler: ruler ?? null,
  };

  await invoke('cheat_set_max_food', { req });
}

export async function cheatSetMaxIron(ruler?: Option<Ruler>) {
  const req: CheatSetMaxIronRequest = {
    world: NIL.world.getIdStrict(),
    ruler: ruler ?? null,
  };

  await invoke('cheat_set_max_iron', { req });
}

export async function cheatSetMaxResources(ruler?: Option<Ruler>) {
  const req: CheatSetMaxResourcesRequest = {
    world: NIL.world.getIdStrict(),
    ruler: ruler ?? null,
  };

  await invoke('cheat_set_max_resources', { req });
}

export async function cheatSetMaxSiloResources(ruler?: Option<Ruler>) {
  const req: CheatSetMaxSiloResourcesRequest = {
    world: NIL.world.getIdStrict(),
    ruler: ruler ?? null,
  };

  await invoke('cheat_set_max_silo_resources', { req });
}

export async function cheatSetMaxStone(ruler?: Option<Ruler>) {
  const req: CheatSetMaxStoneRequest = {
    world: NIL.world.getIdStrict(),
    ruler: ruler ?? null,
  };

  await invoke('cheat_set_max_stone', { req });
}

export async function cheatSetMaxWarehouseResources(ruler?: Option<Ruler>) {
  const req: CheatSetMaxWarehouseResourcesRequest = {
    world: NIL.world.getIdStrict(),
    ruler: ruler ?? null,
  };

  await invoke('cheat_set_max_warehouse_resources', { req });
}

export async function cheatSetMaxWood(ruler?: Option<Ruler>) {
  const req: CheatSetMaxWoodRequest = {
    world: NIL.world.getIdStrict(),
    ruler: ruler ?? null,
  };

  await invoke('cheat_set_max_wood', { req });
}

export async function cheatSetOwnerFood(coord: ContinentKey, food: number) {
  const ruler = await getCityOwner(coord);
  return cheatSetFood(food, ruler);
}

export async function cheatSetOwnerIron(coord: ContinentKey, iron: number) {
  const ruler = await getCityOwner(coord);
  return cheatSetIron(iron, ruler);
}

export async function cheatSetOwnerMaxFood(coord: ContinentKey) {
  const ruler = await getCityOwner(coord);
  return cheatSetMaxFood(ruler);
}

export async function cheatSetOwnerMaxIron(coord: ContinentKey) {
  const ruler = await getCityOwner(coord);
  return cheatSetMaxIron(ruler);
}

export async function cheatSetOwnerMaxResources(coord: ContinentKey) {
  const ruler = await getCityOwner(coord);
  return cheatSetMaxResources(ruler);
}

export async function cheatSetOwnerMaxSiloResources(coord: ContinentKey) {
  const ruler = await getCityOwner(coord);
  return cheatSetMaxSiloResources(ruler);
}

export async function cheatSetOwnerMaxStone(coord: ContinentKey) {
  const ruler = await getCityOwner(coord);
  return cheatSetMaxStone(ruler);
}

export async function cheatSetOwnerMaxWarehouseResources(coord: ContinentKey) {
  const ruler = await getCityOwner(coord);
  return cheatSetMaxWarehouseResources(ruler);
}

export async function cheatSetOwnerMaxWood(coord: ContinentKey) {
  const ruler = await getCityOwner(coord);
  return cheatSetMaxWood(ruler);
}

export async function cheatSetOwnerResources(coord: ContinentKey, resources: Resources) {
  const ruler = await getCityOwner(coord);
  return cheatSetResources(resources, ruler);
}

export async function cheatSetOwnerStone(coord: ContinentKey, stone: number) {
  const ruler = await getCityOwner(coord);
  return cheatSetStone(stone, ruler);
}

export async function cheatSetOwnerWood(coord: ContinentKey, wood: number) {
  const ruler = await getCityOwner(coord);
  return cheatSetWood(wood, ruler);
}

export async function cheatSetResources(
  resources: PartialNullish<Resources>,
  ruler?: Option<Ruler>,
) {
  const req: CheatSetResourcesRequest = {
    world: NIL.world.getIdStrict(),
    resources: ResourcesImpl.create(resources),
    ruler: ruler ?? null,
  };

  await invoke('cheat_set_resources', { req });
}

export async function cheatSetStone(stone: number, ruler?: Option<Ruler>) {
  stone = toU32(stone);
  const req: CheatSetStoneRequest = {
    world: NIL.world.getIdStrict(),
    stone,
    ruler: ruler ?? null,
  };

  await invoke('cheat_set_stone', { req });
}

export async function cheatSetWood(wood: number, ruler?: Option<Ruler>) {
  wood = toU32(wood);
  const req: CheatSetWoodRequest = {
    world: NIL.world.getIdStrict(),
    wood,
    ruler: ruler ?? null,
  };

  await invoke('cheat_set_wood', { req });
}
