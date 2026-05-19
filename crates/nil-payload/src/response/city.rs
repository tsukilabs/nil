// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::{Deref, DerefMut, Display, From, Into};
use nil_core::city::{City, PublicCity};
use nil_core::continent::Coord;
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
pub struct GetCityResponse(pub City);

#[derive(Clone, Copy, Debug, Deref, DerefMut, Display, From, Into, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct GetCityScoreResponse(pub Score);

#[derive(Clone, Debug, Deref, DerefMut, From, Into, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct GetPublicCitiesResponse(pub Vec<GetPublicCityResponse>);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
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

#[derive(Clone, Debug, Deref, DerefMut, From, Into, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct SearchCityResponse(pub Vec<City>);

#[derive(Clone, Debug, Deref, DerefMut, From, Into, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct SearchPublicCityResponse(pub Vec<PublicCity>);
