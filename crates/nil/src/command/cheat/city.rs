// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::city::Stability;
use nil_core::continent::Coord;
use tauri::AppHandle;

#[tauri::command]
pub async fn cheat_set_stability(app: AppHandle, coord: Coord, stability: Stability) -> Result<()> {
  app
    .client(async |cl| {
      cl.cheat_set_stability(coord, stability)
        .await
    })
    .await?
    .map_err(Into::into)
}
