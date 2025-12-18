// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::infrastructure::building::stable::StableRecruitCatalog;
use nil_payload::infrastructure::stable::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn add_stable_recruit_order(
  app: AppHandle,
  req: AddStableRecruitOrderRequest,
) -> Result<()> {
  app
    .client(async |cl| cl.add_stable_recruit_order(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cancel_stable_recruit_order(
  app: AppHandle,
  req: CancelStableRecruitOrderRequest,
) -> Result<()> {
  app
    .client(async |cl| cl.cancel_stable_recruit_order(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_stable_recruit_catalog(
  app: AppHandle,
  req: GetStableRecruitCatalogRequest,
) -> Result<StableRecruitCatalog> {
  app
    .client(async |cl| cl.get_stable_recruit_catalog(req).await)
    .await?
    .map_err(Into::into)
}
