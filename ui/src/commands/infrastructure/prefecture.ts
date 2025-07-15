// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export function addPrefectureBuildOrder(request: PrefectureBuildOrderRequest) {
  return invoke<null>('add_prefecture_build_order', { request });
}

export function cancelPrefectureBuildOrder(coord: Coord) {
  return invoke<null>('cancel_prefecture_build_order', { coord });
}

export function getPrefectureBuildCatalog(coord: Coord) {
  return invoke<PrefectureBuildCatalog>('get_prefecture_build_catalog', { coord });
}
