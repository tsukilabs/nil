// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use nil_core::continent::Coord;
use nil_core::infrastructure::building::academy::{
  AcademyRecruitCatalog,
  AcademyRecruitOrderId,
  AcademyRecruitOrderRequest,
};

impl Client {
  /// POST `/infrastructure/academy/recruit/add`
  pub async fn add_academy_recruit_order(&self, req: AcademyRecruitOrderRequest) -> Result<()> {
    self
      .http
      .post("infrastructure/academy/recruit/add", req)
      .await
  }

  /// POST `/infrastructure/academy/recruit/cancel`
  pub async fn cancel_academy_recruit_order(
    &self,
    coord: Coord,
    id: AcademyRecruitOrderId,
  ) -> Result<()> {
    self
      .http
      .post("infrastructure/academy/recruit/cancel", (coord, id))
      .await
  }

  /// POST `/infrastructure/academy/recruit/catalog`
  pub async fn get_academy_recruit_catalog(&self, coord: Coord) -> Result<AcademyRecruitCatalog> {
    self
      .http
      .post_json("infrastructure/academy/recruit/catalog", coord)
      .await
  }
}
