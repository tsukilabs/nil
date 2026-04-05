// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import type { Coord } from '@/types/core/continent';
import type { InfrastructureQueueOrderId } from '@/types/core/infrastructure/queue';
import type {
  StableRecruitCatalog,
  StableRecruitOrderRequest,
} from '@/types/core/infrastructure/stable';
import type {
  AddStableRecruitOrderRequest,
  CancelStableRecruitOrderRequest,
  GetStableRecruitCatalogRequest,
} from '@/types/request/infrastructure/stable';

export async function addStableRecruitOrder(request: StableRecruitOrderRequest) {
  const req: AddStableRecruitOrderRequest = {
    world: NIL.world.getIdStrict(),
    request,
  };

  await invoke('add_stable_recruit_order', { req });
}

export async function cancelStableRecruitOrder(coord: Coord, id: InfrastructureQueueOrderId) {
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

  return invoke<StableRecruitCatalog>('get_stable_recruit_catalog', { req });
}
