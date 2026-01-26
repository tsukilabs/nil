// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import type { RequestManeuverRequest } from '@/lib/request';

export async function requestManeuver(request: ManeuverRequest) {
  const req: RequestManeuverRequest = {
    world: NIL.world.getIdStrict(),
    request,
  };

  return invoke<ManeuverId>('request_maneuver', { req });
}
