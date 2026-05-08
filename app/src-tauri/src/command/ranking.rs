// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_payload::request::ranking::*;
use nil_payload::response::ranking::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_rank(app: AppHandle, req: GetRankRequest) -> Result<GetRankResponse> {
  app
    .client(async |cl| cl.get_rank(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_ranking(app: AppHandle, req: GetRankingRequest) -> Result<GetRankingResponse> {
  app
    .client(async |cl| cl.get_ranking(req).await)
    .await
    .map_err(Into::into)
}
