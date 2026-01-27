// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export interface GetCityRequest {
  readonly world: WorldId;
  readonly coord: Coord;
}

export interface GetCityScoreRequest {
  readonly world: WorldId;
  readonly coord: Coord;
}

export interface GetPublicCityRequest {
  readonly world: WorldId;
  readonly coord: Coord;
}

export interface RenameCityRequest {
  readonly world: WorldId;
  readonly coord: Coord;
  readonly name: string;
}

export interface SearchCityRequest {
  readonly world: WorldId;
  readonly search: CitySearch;
}

export interface SearchPublicCityRequest {
  readonly world: WorldId;
  readonly search: CitySearch;
}
