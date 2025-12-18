// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::military::maneuver::ManeuverId;
use nil_payload::military::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn request_maneuver(app: AppHandle, req: RequestManeuverRequest) -> Result<ManeuverId> {
  app
    .client(async |cl| cl.request_maneuver(req).await)
    .await?
    .map_err(Into::into)
}
