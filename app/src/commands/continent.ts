// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import { CoordImpl } from '@/core/model/continent/coord';
import type {
  GetContinentSizeRequest,
  GetPublicFieldRequest,
  GetPublicFieldsRequest,
} from '@/lib/request';

export async function getBulkDistance(origin: ContinentKey, destinations: ContinentKey[]) {
  origin = CoordImpl.fromContinentKey(origin);
  destinations = destinations.map((it) => CoordImpl.fromContinentKey(it));

  type Coords = readonly (readonly [Coord, number])[];
  return invoke<Coords>('get_bulk_distance', { origin, destinations });
}

export async function getContinentSize() {
  const req: GetContinentSizeRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<ContinentSize>('get_continent_size', { req });
}

export async function getDistance(origin: ContinentKey, destination: ContinentKey) {
  origin = CoordImpl.fromContinentKey(origin);
  destination = CoordImpl.fromContinentKey(destination);
  return invoke<number>('get_distance', { origin, destination });
}

export async function getField(coord: ContinentKey) {
  const req: GetPublicFieldRequest = {
    world: NIL.world.getIdStrict(),
    coord: CoordImpl.fromContinentKey(coord),
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
