// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import { CoordImpl } from '@/core/model/continent/coord';
import type { GetPublicCityResponse } from '@/lib/response/city';
import type {
  GetCityRequest,
  GetCityScoreRequest,
  GetPublicCitiesRequest,
  GetPublicCityRequest,
  RenameCityRequest,
  SearchCityRequest,
  SearchPublicCityRequest,
} from '@/lib/request';

export async function getCity(coord: ContinentKey) {
  coord = CoordImpl.fromContinentKey(coord);
  const req: GetCityRequest = {
    world: NIL.world.getIdStrict(),
    coord,
  };

  return invoke<City>('get_city', { req });
}

export async function getCityOwner(coord: ContinentKey) {
  return getPublicCity({ coord }).then(({ city }) => city.owner);
}

export async function getCityScore(coord: ContinentKey) {
  coord = CoordImpl.fromContinentKey(coord);
  const req: GetCityScoreRequest = {
    world: NIL.world.getIdStrict(),
    coord,
  };

  return invoke<number>('get_city_score', { req });
}

export async function getPublicCities(options: {
  coords?: Option<ContinentKey[]>;
  score?: Option<boolean>;
  all?: Option<boolean>;
}) {
  const coords = options.coords?.map((coord) => CoordImpl.fromContinentKey(coord));
  const req: GetPublicCitiesRequest = {
    world: NIL.world.getIdStrict(),
    coords: coords ?? [],
    score: options.score ?? false,
    all: options.all ?? false,
  };

  return invoke<readonly GetPublicCityResponse[]>('get_public_cities', { req });
}

export async function getPublicCity(options: {
  coord: ContinentKey;
  score?: Option<boolean>;
}) {
  const req: GetPublicCityRequest = {
    world: NIL.world.getIdStrict(),
    coord: CoordImpl.fromContinentKey(options.coord),
    score: options.score ?? false,
  };

  return invoke<GetPublicCityResponse>('get_public_city', { req });
}

export async function renameCity(coord: ContinentKey, name: string) {
  coord = CoordImpl.fromContinentKey(coord);
  const req: RenameCityRequest = {
    world: NIL.world.getIdStrict(),
    coord,
    name,
  };

  await invoke('rename_city', { req });
}

export async function searchCity(search: CitySearch) {
  const req: SearchCityRequest = {
    world: NIL.world.getIdStrict(),
    search,
  };

  return invoke<readonly City[]>('search_city', { req });
}

export async function searchPublicCity(search: CitySearch) {
  const req: SearchPublicCityRequest = {
    world: NIL.world.getIdStrict(),
    search,
  };

  return invoke<readonly PublicCity[]>('search_public_city', { req });
}
