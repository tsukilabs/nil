// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::continent::Coord;
use nil_core::npc::precursor::{PrecursorId, PublicPrecursor};
use tauri::AppHandle;

#[tauri::command]
pub async fn get_precursor_coords(app: AppHandle, id: PrecursorId) -> Result<Vec<Coord>> {
  app
    .client(async |cl| cl.get_precursor_coords(id).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_public_precursor(app: AppHandle, id: PrecursorId) -> Result<PublicPrecursor> {
  app
    .client(async |cl| cl.get_public_precursor(id).await)
    .await?
    .map_err(Into::into)
}
