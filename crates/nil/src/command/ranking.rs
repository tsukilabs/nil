// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::ranking::{Ranking, RankingEntry};
use nil_core::ruler::Ruler;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_rank(app: AppHandle, ruler: Ruler) -> Result<Option<RankingEntry>> {
  app
    .client(async |cl| cl.get_rank(ruler).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_ranking(app: AppHandle) -> Result<Ranking> {
  app
    .client(async |cl| cl.get_ranking().await)
    .await?
    .map_err(Into::into)
}
