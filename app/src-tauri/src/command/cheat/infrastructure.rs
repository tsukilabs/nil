// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::infrastructure::Infrastructure;
use nil_core::infrastructure::building::academy::AcademyRecruitQueue;
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
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_get_infrastructure(
  app: AppHandle,
  req: CheatGetInfrastructureRequest,
) -> Result<Infrastructure> {
  app
    .client(async |cl| cl.cheat_get_infrastructure(req).await)
    .await?
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
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_get_stable_recruit_queue(
  app: AppHandle,
  req: CheatGetStableRecruitQueueRequest,
) -> Result<StableRecruitQueue> {
  app
    .client(async |cl| cl.cheat_get_stable_recruit_queue(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_get_storage_capacity(
  app: AppHandle,
  req: CheatGetStorageCapacityRequest,
) -> Result<OverallStorageCapacity> {
  app
    .client(async |cl| cl.cheat_get_storage_capacity(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_set_building_level(
  app: AppHandle,
  req: CheatSetBuildingLevelRequest,
) -> Result<()> {
  app
    .client(async |cl| cl.cheat_set_building_level(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_set_max_infrastructure(
  app: AppHandle,
  req: CheatSetMaxInfrastructureRequest,
) -> Result<()> {
  app
    .client(async |cl| cl.cheat_set_max_infrastructure(req).await)
    .await?
    .map_err(Into::into)
}
