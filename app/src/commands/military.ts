// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import type {
  ManeuverRequest,
  RequestManeuverRequest,
  RequestManeuverResponse,
} from '@/types/bindings';

export async function requestManeuver(request: ManeuverRequest) {
  const req: RequestManeuverRequest = {
    world: NIL.world.getIdStrict(),
    request,
  };

  return invoke<RequestManeuverResponse>('request_maneuver', { req });
}
