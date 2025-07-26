// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export function addStableRecruitOrder(request: StableRecruitOrderRequest) {
  return invoke<null>('add_stable_recruit_order', { request });
}

export function cancelStableRecruitOrder(coord: Coord, id: InfrastructureQueueOrderId) {
  return invoke<null>('cancel_stable_recruit_order', { coord, id });
}

export function getStableRecruitCatalog(coord: Coord) {
  return invoke<StableRecruitCatalog>('get_stable_recruit_catalog', { coord });
}
