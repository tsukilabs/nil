// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from "@tauri-apps/api/core";
import type {
  ArmyId,
  GetArmyRequest,
  GetArmyResponse,
  GetManeuverRequest,
  GetManeuverResponse,
  ManeuverId,
  ManeuverRequest,
  RequestManeuverRequest,
  RequestManeuverResponse,
} from "@tsukilabs/nil-bindings";

export async function getArmy(id: ArmyId) {
  const req: GetArmyRequest = {
    world: NIL.world.getIdStrict(),
    id,
  };

  return invoke<GetArmyResponse>("get_army", { req });
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
