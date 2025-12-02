// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export async function requestManeuver(request: ManeuverRequest) {
  return invoke<ManeuverId>('request_maneuver', { req: { request } });
}
