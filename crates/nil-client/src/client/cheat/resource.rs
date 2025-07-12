// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;

use nil_core::resource::prelude::*;

impl Client {
  /// GET `/cheat/resources`
  pub async fn cheat_set_max_resources(&self) -> Result<()> {
    self.http.get("cheat/resources").await
  }

  /// POST `/cheat/resources`
  pub async fn cheat_set_resources(&self, resources: Resources) -> Result<()> {
    self
      .http
      .post("cheat/resources", resources)
      .await
  }

  /// GET `/cheat/resources/food`
  pub async fn cheat_set_max_food(&self) -> Result<()> {
    self.http.get("cheat/resources/food").await
  }

  /// POST `/cheat/resources/food`
  pub async fn cheat_set_food(&self, food: Food) -> Result<()> {
    self
      .http
      .post("cheat/resources/food", food)
      .await
  }

  /// GET `/cheat/resources/iron`
  pub async fn cheat_set_max_iron(&self) -> Result<()> {
    self.http.get("cheat/resources/iron").await
  }

  /// POST `/cheat/resources/iron`
  pub async fn cheat_set_iron(&self, iron: Iron) -> Result<()> {
    self
      .http
      .post("cheat/resources/iron", iron)
      .await
  }

  /// GET `/cheat/resources/silo`
  pub async fn cheat_set_max_silo_resources(&self) -> Result<()> {
    self.http.get("cheat/resources/silo").await
  }

  /// GET `/cheat/resources/stone`
  pub async fn cheat_set_max_stone(&self) -> Result<()> {
    self.http.get("cheat/resources/stone").await
  }

  /// POST `/cheat/resources/stone`
  pub async fn cheat_set_stone(&self, stone: Stone) -> Result<()> {
    self
      .http
      .post("cheat/resources/stone", stone)
      .await
  }

  /// GET `/cheat/resources/warehouse`
  pub async fn cheat_set_max_warehouse_resources(&self) -> Result<()> {
    self
      .http
      .get("cheat/resources/warehouse")
      .await
  }

  /// GET `/cheat/resources/wood`
  pub async fn cheat_set_max_wood(&self) -> Result<()> {
    self.http.get("cheat/resources/wood").await
  }

  /// POST `/cheat/resources/wood`
  pub async fn cheat_set_wood(&self, wood: Wood) -> Result<()> {
    self
      .http
      .post("cheat/resources/wood", wood)
      .await
  }
}
