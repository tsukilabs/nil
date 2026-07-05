// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_payload::request::military::*;
use nil_payload::response::military::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_army(app: AppHandle, req: GetArmyRequest) -> Result<GetArmyResponse> {
  app
    .client(async |cl| cl.get_army(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_army_owner(
  app: AppHandle,
  req: GetArmyOwnerRequest,
) -> Result<GetArmyOwnerResponse> {
  app
    .client(async |cl| cl.get_army_owner(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_maneuver(app: AppHandle, req: GetManeuverRequest) -> Result<GetManeuverResponse> {
  app
    .client(async |cl| cl.get_maneuver(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn request_maneuver(
  app: AppHandle,
  req: RequestManeuverRequest,
) -> Result<RequestManeuverResponse> {
  app
    .client(async |cl| cl.request_maneuver(req).await)
    .await
    .map_err(Into::into)
}
