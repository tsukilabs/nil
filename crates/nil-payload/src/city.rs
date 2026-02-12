// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::city::{CitySearch, PublicCity};
use nil_core::continent::Coord;
use nil_core::error::Error as CoreError;
use nil_core::ranking::Score;
use nil_core::world::World;
use nil_core::world::config::WorldId;
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
pub struct GetPublicCitiesRequest {
  pub world: WorldId,
  #[serde(default)]
  pub coords: Vec<Coord>,
  #[serde(default)]
  pub score: bool,
  #[serde(default)]
  pub all: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPublicCityRequest {
  pub world: WorldId,
  pub coord: Coord,
  #[serde(default)]
  pub score: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPublicCityResponse {
  pub city: PublicCity,
  pub score: Option<Score>,
}

#[bon::bon]
impl GetPublicCityResponse {
  #[builder]
  pub fn new(
    #[builder(start_fn)] world: &World,
    #[builder(start_fn)] coord: Coord,
    #[builder(default)] score: bool,
  ) -> Result<Self, CoreError> {
    let city = world.city(coord)?;
    let score = score
      .then(|| city.score(&world.stats().infrastructure()))
      .transpose()?;

    Ok(Self { city: PublicCity::from(city), score })
  }
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
