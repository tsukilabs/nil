// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use crate::http;
use nil_payload::request::cheat::infrastructure::*;
use nil_payload::response::cheat::infrastructure::*;

impl Client {
  pub async fn cheat_get_academy_recruit_queue(
    &self,
    req: CheatGetAcademyRecruitQueueRequest,
  ) -> Result<CheatGetAcademyRecruitQueueResponse> {
    http::json_put("cheat-get-academy-recruit-queue")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn cheat_get_academy_recruit_queues(
    &self,
    req: CheatGetAcademyRecruitQueuesRequest,
  ) -> Result<CheatGetAcademyRecruitQueuesResponse> {
    http::json_put("cheat-get-academy-recruit-queues")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn cheat_get_all_academy_recruit_queues(
    &self,
    req: CheatGetAllAcademyRecruitQueuesRequest,
  ) -> Result<CheatGetAllAcademyRecruitQueuesResponse> {
    http::json_put("cheat-get-all-academy-recruit-queues")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn cheat_get_all_prefecture_build_queues(
    &self,
    req: CheatGetAllPrefectureBuildQueuesRequest,
  ) -> Result<CheatGetAllPrefectureBuildQueuesResponse> {
    http::json_put("cheat-get-all-prefecture-build-queues")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn cheat_get_all_stable_recruit_queues(
    &self,
    req: CheatGetAllStableRecruitQueuesRequest,
  ) -> Result<CheatGetAllStableRecruitQueuesResponse> {
    http::json_put("cheat-get-all-stable-recruit-queues")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn cheat_get_infrastructure(
    &self,
    req: CheatGetInfrastructureRequest,
  ) -> Result<CheatGetInfrastructureResponse> {
    http::json_put("cheat-get-infrastructure")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn cheat_get_prefecture_build_queue(
    &self,
    req: CheatGetPrefectureBuildQueueRequest,
  ) -> Result<CheatGetPrefectureBuildQueueResponse> {
    http::json_put("cheat-get-prefecture-build-queue")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn cheat_get_prefecture_build_queues(
    &self,
    req: CheatGetPrefectureBuildQueuesRequest,
  ) -> Result<CheatGetPrefectureBuildQueuesResponse> {
    http::json_put("cheat-get-prefecture-build-queues")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn cheat_get_stable_recruit_queue(
    &self,
    req: CheatGetStableRecruitQueueRequest,
  ) -> Result<CheatGetStableRecruitQueueResponse> {
    http::json_put("cheat-get-stable-recruit-queue")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn cheat_get_stable_recruit_queues(
    &self,
    req: CheatGetStableRecruitQueuesRequest,
  ) -> Result<CheatGetStableRecruitQueuesResponse> {
    http::json_put("cheat-get-stable-recruit-queues")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn cheat_get_storage_capacity(
    &self,
    req: CheatGetStorageCapacityRequest,
  ) -> Result<CheatGetStorageCapacityResponse> {
    http::json_put("cheat-get-storage-capacity")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn cheat_set_building_level(&self, req: CheatSetBuildingLevelRequest) -> Result<()> {
    http::post("cheat-set-building-level")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn cheat_set_max_infrastructure(
    &self,
    req: CheatSetMaxInfrastructureRequest,
  ) -> Result<()> {
    http::post("cheat-set-max-infrastructure")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .user_agent(&self.user_agent)
      .send()
      .await
  }
}
