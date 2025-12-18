// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use nil_core::resources::Resources;
use nil_payload::cheat::resources::*;

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

  pub async fn cheat_set_max_food(&self, req: CheatSetMaxFoodRequest) -> Result<()> {
    self
      .http
      .post("cheat-set-max-food", req)
      .await
  }

  pub async fn cheat_set_max_iron(&self, req: CheatSetMaxIronRequest) -> Result<()> {
    self
      .http
      .post("cheat-set-max-iron", req)
      .await
  }

  pub async fn cheat_set_max_resources(&self, req: CheatSetMaxResourcesRequest) -> Result<()> {
    self
      .http
      .post("cheat-set-max-resources", req)
      .await
  }

  pub async fn cheat_set_max_silo_resources(
    &self,
    req: CheatSetMaxSiloResourcesRequest,
  ) -> Result<()> {
    self
      .http
      .post("cheat-set-max-silo-resources", req)
      .await
  }

  pub async fn cheat_set_max_stone(&self, req: CheatSetMaxStoneRequest) -> Result<()> {
    self
      .http
      .post("cheat-set-max-stone", req)
      .await
  }

  pub async fn cheat_set_max_warehouse_resources(
    &self,
    req: CheatSetMaxWarehouseResourcesRequest,
  ) -> Result<()> {
    self
      .http
      .post("cheat-set-max-warehouse-resources", req)
      .await
  }

  pub async fn cheat_set_max_wood(&self, req: CheatSetMaxWoodRequest) -> Result<()> {
    self
      .http
      .post("cheat-set-max-wood", req)
      .await
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
