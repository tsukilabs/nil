// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import type {
  GetContinentSizeRequest,
  GetPublicFieldRequest,
  GetPublicFieldsRequest,
} from '@/lib/request';

export async function getContinentSize() {
  const req: GetContinentSizeRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<ContinentSize>('get_continent_size', { req });
}

export async function getField(coord: Coord) {
  const req: GetPublicFieldRequest = {
    world: NIL.world.getIdStrict(),
    coord,
  };

  return invoke<PublicField>('get_public_field', { req });
}

export async function getFields(coords: Coord[], world?: Option<WorldId>) {
  const req: GetPublicFieldsRequest = {
    world: world ?? NIL.world.getIdStrict(),
    coords,
  };

  type Fields = readonly (readonly [Coord, PublicField])[];
  return invoke<Fields>('get_public_fields', { req });
}
