// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export async function getCity(coord: Coord) {
  return invoke<City>('get_city', { coord });
}

export async function getCityScore(coord: Coord) {
  return invoke<number>('get_city_score', { coord });
}

export async function getPublicCity(coord: Coord) {
  return invoke<PublicCity>('get_public_city', { coord });
}

export async function renameCity(coord: Coord, name: string) {
  return invoke<null>('rename_city', { coord, name });
}
