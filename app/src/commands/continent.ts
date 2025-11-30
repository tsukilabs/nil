// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export async function getField(coord: Coord) {
  return invoke<PublicField>('get_field', { req: { coord } });
}

export async function getFields(coords: Coord[]) {
  type Fields = readonly (readonly [Coord, PublicField])[];
  return invoke<Fields>('get_fields', { req: { coords } });
}

export async function getContinentSize() {
  return invoke<ContinentSize>('get_continent_size');
}
