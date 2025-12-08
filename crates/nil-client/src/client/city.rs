// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::city::{City, PublicCity};
use nil_core::ranking::Score;
use nil_payload::city::{
  GetCityRequest,
  GetCityScoreRequest,
  GetPublicCityRequest,
  RenameCityRequest,
  SearchCityRequest,
  SearchPublicCityRequest,
};

impl Client {
  pub async fn get_city(&self, req: GetCityRequest) -> Result<City> {
    self.http.json_post("get-city", req).await
  }

  pub async fn get_city_score(&self, req: GetCityScoreRequest) -> Result<Score> {
    self
      .http
      .json_post("get-city-score", req)
      .await
  }

  pub async fn get_public_city(&self, req: GetPublicCityRequest) -> Result<PublicCity> {
    self
      .http
      .json_post("get-public-city", req)
      .await
  }

  pub async fn rename_city(&self, req: RenameCityRequest) -> Result<()> {
    self.http.post("rename-city", req).await
  }

  pub async fn search_city(&self, req: SearchCityRequest) -> Result<Vec<City>> {
    self.http.json_post("search-city", req).await
  }

  pub async fn search_public_city(&self, req: SearchPublicCityRequest) -> Result<Vec<PublicCity>> {
    self
      .http
      .json_post("search-public-city", req)
      .await
  }
}
