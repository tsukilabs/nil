// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import type {
  GetPrecursorCoordsRequest,
  GetPublicPrecursorRequest,
  GetPublicPrecursorsRequest,
} from '@/lib/request';

export async function getPrecursorCoords(id: PrecursorId) {
  const req: GetPrecursorCoordsRequest = {
    world: NIL.world.getIdStrict(),
    id,
  };

  return invoke<readonly Coord[]>('get_precursor_coords', { req });
}

export async function getPublicPrecursor(id: PrecursorId) {
  const req: GetPublicPrecursorRequest = {
    world: NIL.world.getIdStrict(),
    id,
  };

  return invoke<PublicPrecursor>('get_public_precursor', { req });
}

export async function getPublicPrecursors() {
  const req: GetPublicPrecursorsRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<readonly PublicPrecursor[]>('get_public_precursors', { req });
}
