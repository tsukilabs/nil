// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use nil_core::infrastructure::Infrastructure;
use nil_core::infrastructure::building::academy::AcademyRecruitQueue;
use nil_core::infrastructure::building::prefecture::PrefectureBuildQueue;
use nil_core::infrastructure::building::stable::StableRecruitQueue;
use nil_core::infrastructure::storage::OverallStorageCapacity;
use nil_payload::cheat::infrastructure::*;

impl Client {
  pub async fn cheat_get_academy_recruit_queue(
    &self,
    req: CheatGetAcademyRecruitQueueRequest,
  ) -> Result<AcademyRecruitQueue> {
    self
      .http
      .json_post("cheat-get-academy-recruit-queue", req)
      .await
  }

  pub async fn cheat_get_infrastructure(
    &self,
    req: CheatGetInfrastructureRequest,
  ) -> Result<Infrastructure> {
    self
      .http
      .json_post("cheat-get-infrastructure", req)
      .await
  }

  pub async fn cheat_get_prefecture_build_queue(
    &self,
    req: CheatGetPrefectureBuildQueueRequest,
  ) -> Result<PrefectureBuildQueue> {
    self
      .http
      .json_post("cheat-get-prefecture-build-queue", req)
      .await
  }

  pub async fn cheat_get_stable_recruit_queue(
    &self,
    req: CheatGetStableRecruitQueueRequest,
  ) -> Result<StableRecruitQueue> {
    self
      .http
      .json_post("cheat-get-stable-recruit-queue", req)
      .await
  }

  pub async fn cheat_get_storage_capacity(
    &self,
    req: CheatGetStorageCapacityRequest,
  ) -> Result<OverallStorageCapacity> {
    self
      .http
      .json_post("cheat-get-storage-capacity", req)
      .await
  }

  pub async fn cheat_set_building_level(&self, req: CheatSetBuildingLevelRequest) -> Result<()> {
    self
      .http
      .post("cheat-set-building-level", req)
      .await
  }

  pub async fn cheat_set_max_infrastructure(
    &self,
    req: CheatSetMaxInfrastructureRequest,
  ) -> Result<()> {
    self
      .http
      .post("cheat-set-max-infrastructure", req)
      .await
  }
}
