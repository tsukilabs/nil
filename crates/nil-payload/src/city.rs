// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::city::CitySearch;
use nil_core::continent::Coord;
use nil_core::world::WorldId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCityRequest {
  pub world: WorldId,
  pub coord: Coord,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCityScoreRequest {
  pub world: WorldId,
  pub coord: Coord,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPublicCityRequest {
  pub world: WorldId,
  pub coord: Coord,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameCityRequest {
  pub world: WorldId,
  pub coord: Coord,
  pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchCityRequest {
  pub world: WorldId,
  pub search: CitySearch,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchPublicCityRequest {
  pub world: WorldId,
  pub search: CitySearch,
}
