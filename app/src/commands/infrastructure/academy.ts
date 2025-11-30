// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export function addAcademyRecruitOrder(request: AcademyRecruitOrderRequest) {
  return invoke<null>('add_academy_recruit_order', { request });
}

export function cancelAcademyRecruitOrder(coord: Coord, id: InfrastructureQueueOrderId) {
  return invoke<null>('cancel_academy_recruit_order', { coord, id });
}

export function getAcademyRecruitCatalog(coord: Coord) {
  return invoke<AcademyRecruitCatalog>('get_academy_recruit_catalog', { coord });
}
