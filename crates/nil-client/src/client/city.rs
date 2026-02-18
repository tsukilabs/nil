// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use crate::http;
use nil_core::city::{City, PublicCity};
use nil_core::ranking::score::Score;
use nil_payload::city::*;

impl Client {
  pub async fn get_city(&self, req: GetCityRequest) -> Result<City> {
    http::json_post("get-city")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn get_city_score(&self, req: GetCityScoreRequest) -> Result<Score> {
    http::json_post("get-city-score")
      .body(req)
      .server(self.server)
      .send()
      .await
  }

  pub async fn get_public_cities(
    &self,
    req: GetPublicCitiesRequest,
  ) -> Result<Vec<GetPublicCityResponse>> {
    http::json_post("get-public-cities")
      .body(req)
      .server(self.server)
      .send()
      .await
  }

  pub async fn get_public_city(&self, req: GetPublicCityRequest) -> Result<GetPublicCityResponse> {
    http::json_post("get-public-city")
      .body(req)
      .server(self.server)
      .send()
      .await
  }

  pub async fn rename_city(&self, req: RenameCityRequest) -> Result<()> {
    http::post("rename-city")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn search_city(&self, req: SearchCityRequest) -> Result<Vec<City>> {
    http::json_post("search-city")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn search_public_city(&self, req: SearchPublicCityRequest) -> Result<Vec<PublicCity>> {
    http::json_post("search-public-city")
      .body(req)
      .server(self.server)
      .send()
      .await
  }
}
