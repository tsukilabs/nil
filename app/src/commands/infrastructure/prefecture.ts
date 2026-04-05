// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import type { Coord } from '@/types/core/continent';
import type {
  PrefectureBuildCatalog,
  PrefectureBuildOrderRequest,
} from '@/types/core/infrastructure/prefecture';
import type {
  AddPrefectureBuildOrderRequest,
  CancelPrefectureBuildOrderRequest,
  GetPrefectureBuildCatalogRequest,
} from '@/types/request/infrastructure/prefecture';

export async function addPrefectureBuildOrder(request: PrefectureBuildOrderRequest) {
  const req: AddPrefectureBuildOrderRequest = {
    world: NIL.world.getIdStrict(),
    request,
  };

  await invoke('add_prefecture_build_order', { req });
}

export async function cancelPrefectureBuildOrder(coord: Coord) {
  const req: CancelPrefectureBuildOrderRequest = {
    world: NIL.world.getIdStrict(),
    coord,
  };

  await invoke('cancel_prefecture_build_order', { req });
}

export async function getPrefectureBuildCatalog(coord: Coord) {
  const req: GetPrefectureBuildCatalogRequest = {
    world: NIL.world.getIdStrict(),
    coord,
  };

  return invoke<PrefectureBuildCatalog>('get_prefecture_build_catalog', { req });
}
