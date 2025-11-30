// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export async function addStableRecruitOrder(request: StableRecruitOrderRequest) {
  await invoke('add_stable_recruit_order', { req: { request } });
}

export async function cancelStableRecruitOrder(coord: Coord, id: InfrastructureQueueOrderId) {
  await invoke('cancel_stable_recruit_order', { req: { coord, id } });
}

export async function getStableRecruitCatalog(coord: Coord) {
  return invoke<StableRecruitCatalog>('get_stable_recruit_catalog', { req: { coord } });
}
