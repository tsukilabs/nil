// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import type {
  AddStableRecruitOrderRequest,
  CancelStableRecruitOrderRequest,
  Coord,
  GetStableRecruitCatalogRequest,
  GetStableRecruitCatalogResponse,
  StableRecruitOrderId,
  StableRecruitOrderRequest,
} from '@/types/bindings';

export async function addStableRecruitOrder(request: StableRecruitOrderRequest) {
  const req: AddStableRecruitOrderRequest = {
    world: NIL.world.getIdStrict(),
    request,
  };

  await invoke('add_stable_recruit_order', { req });
}

export async function cancelStableRecruitOrder(coord: Coord, id: StableRecruitOrderId) {
  const req: CancelStableRecruitOrderRequest = {
    world: NIL.world.getIdStrict(),
    coord,
    id,
  };

  await invoke('cancel_stable_recruit_order', { req });
}

export async function getStableRecruitCatalog(coord: Coord) {
  const req: GetStableRecruitCatalogRequest = {
    world: NIL.world.getIdStrict(),
    coord,
  };

  return invoke<GetStableRecruitCatalogResponse>('get_stable_recruit_catalog', { req });
}
