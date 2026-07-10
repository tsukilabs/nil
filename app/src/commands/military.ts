// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from "@tauri-apps/api/core";
import { CoordImpl } from "@/core/model/continent/coord";
import type { ContinentKey } from "@/types/core/continent";
import type {
  ArmyId,
  CancelManeuverRequest,
  GetArmiesRequest,
  GetArmiesResponse,
  GetArmyOwnerRequest,
  GetArmyOwnerResponse,
  GetArmyRequest,
  GetArmyResponse,
  GetIdleArmiesAtRequest,
  GetIdleArmiesAtResponse,
  GetIdleArmiesCoordsRequest,
  GetIdleArmiesCoordsResponse,
  GetManeuverRequest,
  GetManeuverResponse,
  ManeuverId,
  ManeuverRequest,
  RequestManeuverRequest,
  RequestManeuverResponse,
} from "@tsukilabs/nil-bindings";

export async function cancelManeuver(id: ManeuverId) {
  const req: CancelManeuverRequest = {
    world: NIL.world.getIdStrict(),
    id,
  };

  await invoke("cancel_maneuver", { req });
}

export async function getArmies() {
  const req: GetArmiesRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<GetArmiesResponse>("get_armies", { req });
}

export async function getArmy(id: ArmyId) {
  const req: GetArmyRequest = {
    world: NIL.world.getIdStrict(),
    id,
  };

  return invoke<GetArmyResponse>("get_army", { req });
}

export async function getArmyOwner(id: ArmyId) {
  const req: GetArmyOwnerRequest = {
    world: NIL.world.getIdStrict(),
    id,
  };

  return invoke<GetArmyOwnerResponse>("get_army_owner", { req });
}

export async function getIdleArmiesAt(coord: ContinentKey) {
  coord = CoordImpl.fromContinentKey(coord);
  const req: GetIdleArmiesAtRequest = {
    world: NIL.world.getIdStrict(),
    coord,
  };

  return invoke<GetIdleArmiesAtResponse>("get_idle_armies_at", { req });
}

export async function getIdleArmiesCoords() {
  const req: GetIdleArmiesCoordsRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<GetIdleArmiesCoordsResponse>("get_idle_armies_coords", { req });
}

export async function getManeuver(id: ManeuverId) {
  const req: GetManeuverRequest = {
    world: NIL.world.getIdStrict(),
    id,
  };

  return invoke<GetManeuverResponse>("get_maneuver", { req });
}

export async function requestManeuver(request: ManeuverRequest) {
  const req: RequestManeuverRequest = {
    world: NIL.world.getIdStrict(),
    request,
  };

  return invoke<RequestManeuverResponse>("request_maneuver", { req });
}
