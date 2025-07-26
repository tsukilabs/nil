// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::continent::Coord;
use nil_core::infrastructure::building::stable::{
  StableRecruitCatalog,
  StableRecruitOrderId,
  StableRecruitOrderRequest,
};
use tauri::AppHandle;

#[tauri::command]
pub async fn add_stable_recruit_order(
  app: AppHandle,
  request: StableRecruitOrderRequest,
) -> Result<()> {
  app
    .client(async |cl| cl.add_stable_recruit_order(request).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cancel_stable_recruit_order(
  app: AppHandle,
  coord: Coord,
  id: StableRecruitOrderId,
) -> Result<()> {
  app
    .client(async |cl| {
      cl.cancel_stable_recruit_order(coord, id)
        .await
    })
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_stable_recruit_catalog(
  app: AppHandle,
  coord: Coord,
) -> Result<StableRecruitCatalog> {
  app
    .client(async |cl| cl.get_stable_recruit_catalog(coord).await)
    .await?
    .map_err(Into::into)
}
