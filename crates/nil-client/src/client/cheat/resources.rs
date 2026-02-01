// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use crate::http;
use nil_core::resources::Resources;
use nil_payload::cheat::resources::*;

impl Client {
  pub async fn cheat_get_resources(&self, req: CheatGetResourcesRequest) -> Result<Resources> {
    http::json_post("cheat-get-resources")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn cheat_set_food(&self, req: CheatSetFoodRequest) -> Result<()> {
    http::post("cheat-set-food")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn cheat_set_iron(&self, req: CheatSetIronRequest) -> Result<()> {
    http::post("cheat-set-iron")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn cheat_set_max_food(&self, req: CheatSetMaxFoodRequest) -> Result<()> {
    http::post("cheat-set-max-food")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn cheat_set_max_iron(&self, req: CheatSetMaxIronRequest) -> Result<()> {
    http::post("cheat-set-max-iron")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn cheat_set_max_resources(&self, req: CheatSetMaxResourcesRequest) -> Result<()> {
    http::post("cheat-set-max-resources")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn cheat_set_max_silo_resources(
    &self,
    req: CheatSetMaxSiloResourcesRequest,
  ) -> Result<()> {
    http::post("cheat-set-max-silo-resources")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn cheat_set_max_stone(&self, req: CheatSetMaxStoneRequest) -> Result<()> {
    http::post("cheat-set-max-stone")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn cheat_set_max_warehouse_resources(
    &self,
    req: CheatSetMaxWarehouseResourcesRequest,
  ) -> Result<()> {
    http::post("cheat-set-max-warehouse-resources")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn cheat_set_max_wood(&self, req: CheatSetMaxWoodRequest) -> Result<()> {
    http::post("cheat-set-max-wood")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn cheat_set_resources(&self, req: CheatSetResourcesRequest) -> Result<()> {
    http::post("cheat-set-resources")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn cheat_set_stone(&self, req: CheatSetStoneRequest) -> Result<()> {
    http::post("cheat-set-stone")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn cheat_set_wood(&self, req: CheatSetWoodRequest) -> Result<()> {
    http::post("cheat-set-wood")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }
}
