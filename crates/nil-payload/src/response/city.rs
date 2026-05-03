// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::city::PublicCity;
use nil_core::continent::Coord;
use nil_core::error::Error as CoreError;
use nil_core::ranking::score::Score;
use nil_core::world::World;
use serde::{Deserialize, Serialize};

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
