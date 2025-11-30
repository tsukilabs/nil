// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export async function addAcademyRecruitOrder(request: AcademyRecruitOrderRequest) {
  await invoke('add_academy_recruit_order', { req: { request } });
}

export async function cancelAcademyRecruitOrder(coord: Coord, id: InfrastructureQueueOrderId) {
  await invoke('cancel_academy_recruit_order', { req: { coord, id } });
}

export async function getAcademyRecruitCatalog(coord: Coord) {
  return invoke<AcademyRecruitCatalog>('get_academy_recruit_catalog', { req: { coord } });
}
