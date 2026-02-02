// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { toU8 } from '@/lib/number';
import { invoke } from '@tauri-apps/api/core';
import { getCityOwner } from '@/commands/city';
import { getBotCoords } from '@/commands/npc/bot';
import { getRulerCoords } from '@/commands/ruler';
import { getPlayerCoords } from '@/commands/player';
import { CoordImpl } from '@/core/model/continent/coord';
import { getPrecursorCoords } from '@/commands/npc/precursor';
import type {
  CheatGetAcademyRecruitQueueRequest,
  CheatGetAcademyRecruitQueuesRequest,
  CheatGetAllAcademyRecruitQueuesRequest,
  CheatGetAllPrefectureBuildQueuesRequest,
  CheatGetAllStableRecruitQueuesRequest,
  CheatGetInfrastructureRequest,
  CheatGetPrefectureBuildQueueRequest,
  CheatGetPrefectureBuildQueuesRequest,
  CheatGetStableRecruitQueueRequest,
  CheatGetStableRecruitQueuesRequest,
  CheatGetStorageCapacityRequest,
  CheatSetBuildingLevelRequest,
  CheatSetMaxInfrastructureRequest,
} from '@/lib/request';

export async function cheatGetAcademyRecruitQueue(coord?: Option<ContinentKey>) {
  coord = CoordImpl.fromContinentKeyOrCurrentStrict(coord);
  const req: CheatGetAcademyRecruitQueueRequest = {
    world: NIL.world.getIdStrict(),
    coord,
  };

  return invoke<AcademyRecruitQueue>('cheat_get_academy_recruit_queue', { req });
}

export async function cheatGetAcademyRecruitQueues(
  coords: readonly Option<ContinentKey>[] = [],
  filterEmpty?: Option<boolean>,
) {
  const req: CheatGetAcademyRecruitQueuesRequest = {
    world: NIL.world.getIdStrict(),
    coords: coords.map((coord) => CoordImpl.fromContinentKeyStrict(coord)),
    filterEmpty: filterEmpty ?? false,
  };

  type Queues = readonly (readonly [Coord, AcademyRecruitQueue])[];
  return invoke<Queues>('cheat_get_academy_recruit_queues', { req });
}

export async function cheatGetAllAcademyRecruitQueues(filterEmpty?: Option<boolean>) {
  const req: CheatGetAllAcademyRecruitQueuesRequest = {
    world: NIL.world.getIdStrict(),
    filterEmpty: filterEmpty ?? false,
  };

  type Queues = readonly (readonly [Coord, AcademyRecruitQueue])[];
  return invoke<Queues>('cheat_get_all_academy_recruit_queues', { req });
}

export async function cheatGetAllPrefectureBuildQueues(filterEmpty?: Option<boolean>) {
  const req: CheatGetAllPrefectureBuildQueuesRequest = {
    world: NIL.world.getIdStrict(),
    filterEmpty: filterEmpty ?? false,
  };

  type Queues = readonly (readonly [Coord, PrefectureBuildQueue])[];
  return invoke<Queues>('cheat_get_all_prefecture_build_queues', { req });
}

export async function cheatGetAllStableRecruitQueues(filterEmpty?: Option<boolean>) {
  const req: CheatGetAllStableRecruitQueuesRequest = {
    world: NIL.world.getIdStrict(),
    filterEmpty: filterEmpty ?? false,
  };

  type Queues = readonly (readonly [Coord, StableRecruitQueue])[];
  return invoke<Queues>('cheat_get_all_stable_recruit_queues', { req });
}

export async function cheatGetInfrastructure(coord?: Option<ContinentKey>) {
  coord = CoordImpl.fromContinentKeyOrCurrentStrict(coord);
  const req: CheatGetInfrastructureRequest = {
    world: NIL.world.getIdStrict(),
    coord,
  };

  return invoke<Infrastructure>('cheat_get_infrastructure', { req });
}

export async function cheatGetPrefectureBuildQueue(coord?: Option<ContinentKey>) {
  coord = CoordImpl.fromContinentKeyOrCurrentStrict(coord);
  const req: CheatGetPrefectureBuildQueueRequest = {
    world: NIL.world.getIdStrict(),
    coord,
  };

  return invoke<PrefectureBuildQueue>('cheat_get_prefecture_build_queue', { req });
}

export async function cheatGetPrefectureBuildQueues(
  coords: readonly Option<ContinentKey>[] = [],
  filterEmpty?: Option<boolean>,
) {
  const req: CheatGetPrefectureBuildQueuesRequest = {
    world: NIL.world.getIdStrict(),
    coords: coords.map((coord) => CoordImpl.fromContinentKeyStrict(coord)),
    filterEmpty: filterEmpty ?? false,
  };

  type Queues = readonly (readonly [Coord, PrefectureBuildQueue])[];
  return invoke<Queues>('cheat_get_prefecture_build_queues', { req });
}

export async function cheatGetBotAcademyRecruitQueues(
  id: BotId,
  filterEmpty?: Option<boolean>,
) {
  const coords = await getBotCoords(id);
  return cheatGetAcademyRecruitQueues(coords, filterEmpty);
}

export async function cheatGetBotPrefectureBuildQueues(
  id: BotId,
  filterEmpty?: Option<boolean>,
) {
  const coords = await getBotCoords(id);
  return cheatGetPrefectureBuildQueues(coords, filterEmpty);
}

export async function cheatGetBotStableRecruitQueues(
  id: BotId,
  filterEmpty?: Option<boolean>,
) {
  const coords = await getBotCoords(id);
  return cheatGetStableRecruitQueues(coords, filterEmpty);
}

export async function cheatGetPlayerAcademyRecruitQueues(
  id: PlayerId,
  filterEmpty?: Option<boolean>,
) {
  const coords = await getPlayerCoords(id);
  return cheatGetAcademyRecruitQueues(coords, filterEmpty);
}

export async function cheatGetPlayerPrefectureBuildQueues(
  id: PlayerId,
  filterEmpty?: Option<boolean>,
) {
  const coords = await getPlayerCoords(id);
  return cheatGetPrefectureBuildQueues(coords, filterEmpty);
}

export async function cheatGetPlayerStableRecruitQueues(
  id: PlayerId,
  filterEmpty?: Option<boolean>,
) {
  const coords = await getPlayerCoords(id);
  return cheatGetStableRecruitQueues(coords, filterEmpty);
}

export async function cheatGetPrecursorAcademyRecruitQueues(
  id: PrecursorId,
  filterEmpty?: Option<boolean>,
) {
  const coords = await getPrecursorCoords(id);
  return cheatGetAcademyRecruitQueues(coords, filterEmpty);
}

export async function cheatGetPrecursorPrefectureBuildQueues(
  id: PrecursorId,
  filterEmpty?: Option<boolean>,
) {
  const coords = await getPrecursorCoords(id);
  return cheatGetPrefectureBuildQueues(coords, filterEmpty);
}

export async function cheatGetPrecursorStableRecruitQueues(
  id: PrecursorId,
  filterEmpty?: Option<boolean>,
) {
  const coords = await getPrecursorCoords(id);
  return cheatGetStableRecruitQueues(coords, filterEmpty);
}

export async function cheatGetRulerAcademyRecruitQueues(
  ruler: Ruler,
  filterEmpty?: Option<boolean>,
) {
  const coords = await getRulerCoords(ruler);
  return cheatGetAcademyRecruitQueues(coords, filterEmpty);
}

export async function cheatGetRulerPrefectureBuildQueues(
  ruler: Ruler,
  filterEmpty?: Option<boolean>,
) {
  const coords = await getRulerCoords(ruler);
  return cheatGetPrefectureBuildQueues(coords, filterEmpty);
}

export async function cheatGetRulerStableRecruitQueues(
  ruler: Ruler,
  filterEmpty?: Option<boolean>,
) {
  const coords = await getRulerCoords(ruler);
  return cheatGetStableRecruitQueues(coords, filterEmpty);
}

export async function cheatGetOwnerAcademyRecruitQueues(
  coord: ContinentKey,
  filterEmpty?: Option<boolean>,
) {
  const owner = await getCityOwner(coord);
  return cheatGetRulerAcademyRecruitQueues(owner, filterEmpty);
}

export async function cheatGetOwnerPrefectureBuildQueues(
  coord: ContinentKey,
  filterEmpty?: Option<boolean>,
) {
  const owner = await getCityOwner(coord);
  return cheatGetRulerPrefectureBuildQueues(owner, filterEmpty);
}

export async function cheatGetOwnerStableRecruitQueues(
  coord: ContinentKey,
  filterEmpty?: Option<boolean>,
) {
  const owner = await getCityOwner(coord);
  return cheatGetRulerStableRecruitQueues(owner, filterEmpty);
}

export async function cheatGetStableRecruitQueue(coord?: Option<ContinentKey>) {
  coord = CoordImpl.fromContinentKeyOrCurrentStrict(coord);
  const req: CheatGetStableRecruitQueueRequest = {
    world: NIL.world.getIdStrict(),
    coord,
  };

  return invoke<StableRecruitQueue>('cheat_get_stable_recruit_queue', { req });
}

export async function cheatGetStableRecruitQueues(
  coords: readonly Option<ContinentKey>[] = [],
  filterEmpty?: Option<boolean>,
) {
  const req: CheatGetStableRecruitQueuesRequest = {
    world: NIL.world.getIdStrict(),
    coords: coords.map((coord) => CoordImpl.fromContinentKeyStrict(coord)),
    filterEmpty: filterEmpty ?? false,
  };

  type Queues = readonly (readonly [Coord, StableRecruitQueue])[];
  return invoke<Queues>('cheat_get_stable_recruit_queues', { req });
}

export async function cheatGetStorageCapacity(ruler?: Option<Ruler>) {
  const req: CheatGetStorageCapacityRequest = {
    world: NIL.world.getIdStrict(),
    ruler: ruler ?? null,
  };

  return invoke<OverallStorageCapacity>('cheat_get_storage_capacity', { req });
}

export async function cheatGetOwnerStorageCapacity(coord: ContinentKey) {
  const ruler = await getCityOwner(coord);
  return cheatGetStorageCapacity(ruler);
}

export async function cheatSetBuildingLevel(
  coord: ContinentKey,
  id: BuildingId,
  level: BuildingLevel,
) {
  coord = CoordImpl.fromContinentKey(coord);
  level = toU8(level);

  const req: CheatSetBuildingLevelRequest = {
    world: NIL.world.getIdStrict(),
    coord,
    id,
    level,
  };

  await invoke('cheat_set_building_level', { req });
}

export async function cheatSetMaxInfrastructure(coord?: Option<ContinentKey>) {
  coord = CoordImpl.fromContinentKeyOrCurrentStrict(coord);
  const req: CheatSetMaxInfrastructureRequest = {
    world: NIL.world.getIdStrict(),
    coord,
  };

  await invoke('cheat_set_max_infrastructure', { req });
}
