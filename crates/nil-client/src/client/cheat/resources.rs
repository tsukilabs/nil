// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use nil_core::resources::Resources;
use nil_payload::cheat::resources::{
  CheatGetResourcesRequest,
  CheatSetFoodRequest,
  CheatSetIronRequest,
  CheatSetResourcesRequest,
  CheatSetStoneRequest,
  CheatSetWoodRequest,
};

impl Client {
  pub async fn cheat_get_resources(&self, req: CheatGetResourcesRequest) -> Result<Resources> {
    self
      .http
      .json_post("cheat-get-resources", req)
      .await
  }

  pub async fn cheat_set_food(&self, req: CheatSetFoodRequest) -> Result<()> {
    self.http.post("cheat-set-food", req).await
  }

  pub async fn cheat_set_iron(&self, req: CheatSetIronRequest) -> Result<()> {
    self.http.post("cheat-set-iron", req).await
  }

  pub async fn cheat_set_max_food(&self) -> Result<()> {
    self.http.get("cheat-set-max-food").await
  }

  pub async fn cheat_set_max_iron(&self) -> Result<()> {
    self.http.get("cheat-set-max-iron").await
  }

  pub async fn cheat_set_max_resources(&self) -> Result<()> {
    self
      .http
      .get("cheat-set-max-resources")
      .await
  }

  pub async fn cheat_set_max_silo_resources(&self) -> Result<()> {
    self
      .http
      .get("cheat-set-max-silo-resources")
      .await
  }

  pub async fn cheat_set_max_stone(&self) -> Result<()> {
    self.http.get("cheat-set-max-stone").await
  }

  pub async fn cheat_set_max_warehouse_resources(&self) -> Result<()> {
    self
      .http
      .get("cheat-set-max-warehouse-resources")
      .await
  }

  pub async fn cheat_set_max_wood(&self) -> Result<()> {
    self.http.get("cheat-set-max-wood").await
  }

  pub async fn cheat_set_resources(&self, req: CheatSetResourcesRequest) -> Result<()> {
    self
      .http
      .post("cheat-set-resources", req)
      .await
  }

  pub async fn cheat_set_stone(&self, req: CheatSetStoneRequest) -> Result<()> {
    self.http.post("cheat-set-stone", req).await
  }

  pub async fn cheat_set_wood(&self, req: CheatSetWoodRequest) -> Result<()> {
    self.http.post("cheat-set-wood", req).await
  }
}
