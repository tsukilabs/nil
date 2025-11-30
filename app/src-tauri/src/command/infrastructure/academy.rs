// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::infrastructure::building::academy::AcademyRecruitCatalog;
use nil_payload::infrastructure::academy::{
  AddAcademyRecruitOrderRequest,
  CancelAcademyRecruitOrderRequest,
  GetAcademyRecruitCatalogRequest,
};
use tauri::AppHandle;

#[tauri::command]
pub async fn add_academy_recruit_order(
  app: AppHandle,
  req: AddAcademyRecruitOrderRequest,
) -> Result<()> {
  app
    .client(async |cl| cl.add_academy_recruit_order(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cancel_academy_recruit_order(
  app: AppHandle,
  req: CancelAcademyRecruitOrderRequest,
) -> Result<()> {
  app
    .client(async |cl| cl.cancel_academy_recruit_order(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_academy_recruit_catalog(
  app: AppHandle,
  req: GetAcademyRecruitCatalogRequest,
) -> Result<AcademyRecruitCatalog> {
  app
    .client(async |cl| cl.get_academy_recruit_catalog(req).await)
    .await?
    .map_err(Into::into)
}
