// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use bon::Builder;
use nil_core::city::CitySearch;
use nil_core::continent::coord::Coord;
use nil_core::world::config::WorldId;
use serde::{Deserialize, Serialize};

#[cfg(feature = "typescript")]
use ts_rs::TS;

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct GetCityRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
  #[builder(into)]
  pub coord: Coord,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct GetCityScoreRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
  #[builder(into)]
  pub coord: Coord,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct GetPublicCitiesRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
  #[serde(default)]
  #[builder(default, with = FromIterator::from_iter)]
  pub coords: Vec<Coord>,
  #[serde(default)]
  #[builder(default)]
  pub score: bool,
  #[serde(default)]
  #[builder(default)]
  pub all: bool,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct GetPublicCityRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
  #[builder(into)]
  pub coord: Coord,
  #[serde(default)]
  #[builder(default)]
  pub score: bool,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct RenameCityRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
  #[builder(into)]
  pub coord: Coord,
  #[builder(into)]
  pub name: String,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct SearchCityRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
  #[builder(into)]
  pub search: CitySearch,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct SearchPublicCityRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
  #[builder(into)]
  pub search: CitySearch,
}
