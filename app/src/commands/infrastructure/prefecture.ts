// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import type {
  AddPrefectureBuildOrderRequest,
  CancelPrefectureBuildOrderRequest,
  Coord,
  GetPrefectureBuildCatalogRequest,
  GetPrefectureBuildCatalogResponse,
  PrefectureBuildOrderRequest,
} from '@tsukilabs/nil-bindings';

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

  return invoke<GetPrefectureBuildCatalogResponse>('get_prefecture_build_catalog', { req });
}
