// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use itertools::Itertools;
use nil_payload::request::cheat::infrastructure::*;
use nil_payload::response::cheat::infrastructure::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn cheat_get_academy_recruit_queue(
  app: AppHandle,
  req: CheatGetAcademyRecruitQueueRequest,
) -> Result<CheatGetAcademyRecruitQueueResponse> {
  app
    .client(async |cl| cl.cheat_get_academy_recruit_queue(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_get_academy_recruit_queues(
  app: AppHandle,
  mut req: CheatGetAcademyRecruitQueuesRequest,
) -> Result<CheatGetAcademyRecruitQueuesResponse> {
  req.coords = req.coords.into_iter().unique().collect();
  app
    .client(async |cl| {
      cl.cheat_get_academy_recruit_queues(req)
        .await
    })
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_get_all_academy_recruit_queues(
  app: AppHandle,
  req: CheatGetAllAcademyRecruitQueuesRequest,
) -> Result<CheatGetAllAcademyRecruitQueuesResponse> {
  app
    .client(async |cl| {
      cl.cheat_get_all_academy_recruit_queues(req)
        .await
    })
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_get_all_prefecture_build_queues(
  app: AppHandle,
  req: CheatGetAllPrefectureBuildQueuesRequest,
) -> Result<CheatGetAllPrefectureBuildQueuesResponse> {
  app
    .client(async |cl| {
      cl.cheat_get_all_prefecture_build_queues(req)
        .await
    })
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_get_all_stable_recruit_queues(
  app: AppHandle,
  req: CheatGetAllStableRecruitQueuesRequest,
) -> Result<CheatGetAllStableRecruitQueuesResponse> {
  app
    .client(async |cl| {
      cl.cheat_get_all_stable_recruit_queues(req)
        .await
    })
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_get_infrastructure(
  app: AppHandle,
  req: CheatGetInfrastructureRequest,
) -> Result<CheatGetInfrastructureResponse> {
  app
    .client(async |cl| cl.cheat_get_infrastructure(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_get_prefecture_build_queue(
  app: AppHandle,
  req: CheatGetPrefectureBuildQueueRequest,
) -> Result<CheatGetPrefectureBuildQueueResponse> {
  app
    .client(async |cl| {
      cl.cheat_get_prefecture_build_queue(req)
        .await
    })
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_get_prefecture_build_queues(
  app: AppHandle,
  mut req: CheatGetPrefectureBuildQueuesRequest,
) -> Result<CheatGetPrefectureBuildQueuesResponse> {
  req.coords = req.coords.into_iter().unique().collect();
  app
    .client(async |cl| {
      cl.cheat_get_prefecture_build_queues(req)
        .await
    })
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_get_stable_recruit_queue(
  app: AppHandle,
  req: CheatGetStableRecruitQueueRequest,
) -> Result<CheatGetStableRecruitQueueResponse> {
  app
    .client(async |cl| cl.cheat_get_stable_recruit_queue(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_get_stable_recruit_queues(
  app: AppHandle,
  mut req: CheatGetStableRecruitQueuesRequest,
) -> Result<CheatGetStableRecruitQueuesResponse> {
  req.coords = req.coords.into_iter().unique().collect();
  app
    .client(async |cl| cl.cheat_get_stable_recruit_queues(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_get_storage_capacity(
  app: AppHandle,
  req: CheatGetStorageCapacityRequest,
) -> Result<CheatGetStorageCapacityResponse> {
  app
    .client(async |cl| cl.cheat_get_storage_capacity(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_set_building_level(
  app: AppHandle,
  req: CheatSetBuildingLevelRequest,
) -> Result<()> {
  app
    .client(async |cl| cl.cheat_set_building_level(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_set_max_infrastructure(
  app: AppHandle,
  req: CheatSetMaxInfrastructureRequest,
) -> Result<()> {
  app
    .client(async |cl| cl.cheat_set_max_infrastructure(req).await)
    .await
    .map_err(Into::into)
}
