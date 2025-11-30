// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export async function addPrefectureBuildOrder(request: PrefectureBuildOrderRequest) {
  await invoke('add_prefecture_build_order', { req: { request } });
}

export async function cancelPrefectureBuildOrder(coord: Coord) {
  await invoke('cancel_prefecture_build_order', { req: { coord } });
}

export async function getPrefectureBuildCatalog(coord: Coord) {
  return invoke<PrefectureBuildCatalog>('get_prefecture_build_catalog', { req: { coord } });
}
