// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import type {
  AddWorkshopRecruitOrderRequest,
  CancelWorkshopRecruitOrderRequest,
  GetWorkshopRecruitCatalogRequest,
} from '@/lib/request';

export async function addWorkshopRecruitOrder(request: WorkshopRecruitOrderRequest) {
  const req: AddWorkshopRecruitOrderRequest = {
    world: NIL.world.getIdStrict(),
    request,
  };

  await invoke('add_workshop_recruit_order', { req });
}

export async function cancelWorkshopRecruitOrder(coord: Coord, id: InfrastructureQueueOrderId) {
  const req: CancelWorkshopRecruitOrderRequest = {
    world: NIL.world.getIdStrict(),
    coord,
    id,
  };

  await invoke('cancel_workshop_recruit_order', { req });
}

export async function getWorkshopRecruitCatalog(coord: Coord) {
  const req: GetWorkshopRecruitCatalogRequest = {
    world: NIL.world.getIdStrict(),
    coord,
  };

  return invoke<WorkshopRecruitCatalog>('get_workshop_recruit_catalog', { req });
}
