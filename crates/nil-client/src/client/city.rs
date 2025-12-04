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
  /// POST `/city`
  pub async fn get_city(&self, req: GetCityRequest) -> Result<City> {
    self.http.post_json("city", req).await
  }

  /// POST `/city/public`
  pub async fn get_public_city(&self, req: GetPublicCityRequest) -> Result<PublicCity> {
    self.http.post_json("city/public", req).await
  }

  /// POST `/city/public/search`
  pub async fn search_public_city(&self, req: SearchPublicCityRequest) -> Result<Vec<PublicCity>> {
    self
      .http
      .post_json("city/public/search", req)
      .await
  }

  /// POST `/city/rename`
  pub async fn rename_city(&self, req: RenameCityRequest) -> Result<()> {
    self.http.post("city/rename", req).await
  }

  /// POST `/city/score`
  pub async fn get_city_score(&self, req: GetCityScoreRequest) -> Result<Score> {
    self.http.post_json("city/score", req).await
  }

  /// POST `/city/search`
  pub async fn search_city(&self, req: SearchCityRequest) -> Result<Vec<City>> {
    self.http.post_json("city/search", req).await
  }
}
