// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export function addPrefectureBuildOrder(options: PrefectureBuildOrderOptions) {
  return invoke<null>('add_prefecture_build_order', { options });
}

export function getPrefectureCatalog(coord: Coord) {
  return invoke<PrefectureCatalog>('get_prefecture_catalog', { coord });
}
