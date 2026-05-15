// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from "@tauri-apps/api/core";
import type {
  AddWorkshopRecruitOrderRequest,
  CancelWorkshopRecruitOrderRequest,
  Coord,
  GetWorkshopRecruitCatalogRequest,
  GetWorkshopRecruitCatalogResponse,
  WorkshopRecruitOrderId,
  WorkshopRecruitOrderRequest,
} from "@tsukilabs/nil-bindings";

export async function addWorkshopRecruitOrder(request: WorkshopRecruitOrderRequest) {
  const req: AddWorkshopRecruitOrderRequest = {
    world: NIL.world.getIdStrict(),
    request,
  };

  await invoke("add_workshop_recruit_order", { req });
}

export async function cancelWorkshopRecruitOrder(coord: Coord, id: WorkshopRecruitOrderId) {
  const req: CancelWorkshopRecruitOrderRequest = {
    world: NIL.world.getIdStrict(),
    coord,
    id,
  };

  await invoke("cancel_workshop_recruit_order", { req });
}

export async function getWorkshopRecruitCatalog(coord: Coord) {
  const req: GetWorkshopRecruitCatalogRequest = {
    world: NIL.world.getIdStrict(),
    coord,
  };

  return invoke<GetWorkshopRecruitCatalogResponse>("get_workshop_recruit_catalog", { req });
}
