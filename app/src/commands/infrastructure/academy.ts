// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import type {
  AddAcademyRecruitOrderRequest,
  CancelAcademyRecruitOrderRequest,
  GetAcademyRecruitCatalogRequest,
} from '@/lib/request';

export async function addAcademyRecruitOrder(request: AcademyRecruitOrderRequest) {
  const req: AddAcademyRecruitOrderRequest = {
    world: NIL.world.getIdStrict(),
    request,
  };

  await invoke('add_academy_recruit_order', { req });
}

export async function cancelAcademyRecruitOrder(coord: Coord, id: InfrastructureQueueOrderId) {
  const req: CancelAcademyRecruitOrderRequest = {
    world: NIL.world.getIdStrict(),
    coord,
    id,
  };

  await invoke('cancel_academy_recruit_order', { req });
}

export async function getAcademyRecruitCatalog(coord: Coord) {
  const req: GetAcademyRecruitCatalogRequest = {
    world: NIL.world.getIdStrict(),
    coord,
  };

  return invoke<AcademyRecruitCatalog>('get_academy_recruit_catalog', { req });
}
