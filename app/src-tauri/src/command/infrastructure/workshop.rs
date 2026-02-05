// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::infrastructure::building::workshop::recruit_catalog::WorkshopRecruitCatalog;
use nil_payload::infrastructure::workshop::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn add_workshop_recruit_order(
  app: AppHandle,
  req: AddWorkshopRecruitOrderRequest,
) -> Result<()> {
  app
    .client(async |cl| cl.add_workshop_recruit_order(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cancel_workshop_recruit_order(
  app: AppHandle,
  req: CancelWorkshopRecruitOrderRequest,
) -> Result<()> {
  app
    .client(async |cl| cl.cancel_workshop_recruit_order(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_workshop_recruit_catalog(
  app: AppHandle,
  req: GetWorkshopRecruitCatalogRequest,
) -> Result<WorkshopRecruitCatalog> {
  app
    .client(async |cl| cl.get_workshop_recruit_catalog(req).await)
    .await
    .map_err(Into::into)
}
