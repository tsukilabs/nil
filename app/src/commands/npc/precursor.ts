// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export async function getPrecursorCoords(id: PrecursorId) {
  return invoke<readonly Coord[]>('get_precursor_coords', { id });
}

export function getPublicPrecursor(id: PrecursorId) {
  return invoke<PublicPrecursor>('get_public_precursor', { id });
}
