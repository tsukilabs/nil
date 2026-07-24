// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::response::city::GetCityResponse;
use derive_more::{Deref, DerefMut, From, Into};
use nil_core::city::City;
use nil_core::continent::coord::Coord;
use nil_core::error::Error as CoreError;
use nil_core::ranking::score::Score;
use nil_core::world::World;
use serde::{Deserialize, Serialize};

#[cfg(feature = "axum")]
use nil_payload_macros::IntoJsonResponse;

#[cfg(feature = "typescript")]
use ts_rs::TS;

#[derive(Clone, Debug, Deref, DerefMut, From, Into, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetCitiesResponse(pub Vec<CheatGetCityResponse>);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetCityResponse {
  pub city: City,
  pub score: Option<Score>,
}

#[bon::bon]
impl CheatGetCityResponse {
  #[builder]
  pub fn new(
    #[builder(start_fn)] world: &World,
    #[builder(start_fn)] coord: Coord,
    #[builder(default)] score: bool,
  ) -> Result<Self, CoreError> {
    let city = world.cheat_get_city(coord)?;
    let score = score
      .then(|| city.score(&world.stats().infrastructure()))
      .transpose()?;

    Ok(Self { city: city.clone(), score })
  }
}

impl From<GetCityResponse> for CheatGetCityResponse {
  fn from(value: GetCityResponse) -> Self {
    Self { city: value.city, score: value.score }
  }
}
