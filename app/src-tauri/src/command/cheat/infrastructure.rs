// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use itertools::Itertools;
use nil_core::continent::Coord;
use nil_core::infrastructure::Infrastructure;
use nil_core::infrastructure::building::academy::recruit_queue::AcademyRecruitQueue;
use nil_core::infrastructure::building::prefecture::PrefectureBuildQueue;
use nil_core::infrastructure::building::stable::StableRecruitQueue;
use nil_core::infrastructure::storage::OverallStorageCapacity;
use nil_payload::cheat::infrastructure::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn cheat_get_academy_recruit_queue(
  app: AppHandle,
  req: CheatGetAcademyRecruitQueueRequest,
) -> Result<AcademyRecruitQueue> {
  app
    .client(async |cl| cl.cheat_get_academy_recruit_queue(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_get_academy_recruit_queues(
  app: AppHandle,
  mut req: CheatGetAcademyRecruitQueuesRequest,
) -> Result<Vec<(Coord, AcademyRecruitQueue)>> {
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
) -> Result<Vec<(Coord, AcademyRecruitQueue)>> {
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
) -> Result<Vec<(Coord, PrefectureBuildQueue)>> {
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
) -> Result<Vec<(Coord, StableRecruitQueue)>> {
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
) -> Result<Infrastructure> {
  app
    .client(async |cl| cl.cheat_get_infrastructure(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_get_prefecture_build_queue(
  app: AppHandle,
  req: CheatGetPrefectureBuildQueueRequest,
) -> Result<PrefectureBuildQueue> {
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
) -> Result<Vec<(Coord, PrefectureBuildQueue)>> {
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
) -> Result<StableRecruitQueue> {
  app
    .client(async |cl| cl.cheat_get_stable_recruit_queue(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_get_stable_recruit_queues(
  app: AppHandle,
  mut req: CheatGetStableRecruitQueuesRequest,
) -> Result<Vec<(Coord, StableRecruitQueue)>> {
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
) -> Result<OverallStorageCapacity> {
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
