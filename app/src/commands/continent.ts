// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export function getField(coord: Coord) {
  return invoke<PublicField>('get_field', { coord });
}

export function getFields(coords: Coord[]) {
  type Fields = [Coord, PublicField][];
  return invoke<Fields>('get_fields', { coords });
}

export function getContinentSize() {
  return invoke<ContinentSize>('get_continent_size');
}
