// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import { CoordImpl } from '@/core/model/continent/coord';

export async function getCity(coord: ContinentKey) {
  coord = CoordImpl.fromContinentKey(coord);
  return invoke<City>('get_city', { req: { coord } });
}

export async function getCityOwner(coord: ContinentKey) {
  return getPublicCity(coord).then((city) => city.owner);
}

export async function getCityScore(coord: ContinentKey) {
  coord = CoordImpl.fromContinentKey(coord);
  return invoke<number>('get_city_score', { req: { coord } });
}

export async function getPublicCity(coord: ContinentKey) {
  coord = CoordImpl.fromContinentKey(coord);
  return invoke<PublicCity>('get_public_city', { req: { coord } });
}

export async function renameCity(coord: ContinentKey, name: string) {
  coord = CoordImpl.fromContinentKey(coord);
  await invoke('rename_city', { req: { coord, name } });
}

export async function searchCity(search: CitySearch) {
  return invoke<readonly City[]>('search_city', { req: { search } });
}

export async function searchPublicCity(search: CitySearch) {
  return invoke<readonly PublicCity[]>('search_public_city', { req: { search } });
}
