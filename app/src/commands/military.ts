// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import type { RequestManeuverRequest } from '@/types/request/military';
import type { ManeuverId, ManeuverRequest } from '@/types/core/military/maneuver';

export async function requestManeuver(request: ManeuverRequest) {
  const req: RequestManeuverRequest = {
    world: NIL.world.getIdStrict(),
    request,
  };

  return invoke<ManeuverId>('request_maneuver', { req });
}
