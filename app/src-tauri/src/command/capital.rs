// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_payload::request::capital::*;
use nil_payload::response::capital::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_city_limit(
  app: AppHandle,
  req: GetCityLimitRequest,
) -> Result<GetCityLimitResponse> {
  app
    .client(async |cl| cl.get_city_limit(req).await)
    .await
    .map_err(Into::into)
}
