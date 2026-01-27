// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::continent::Coord;
use nil_core::npc::precursor::PublicPrecursor;
use nil_payload::npc::precursor::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_precursor_coords(
  app: AppHandle,
  req: GetPrecursorCoordsRequest,
) -> Result<Vec<Coord>> {
  app
    .client(async |cl| cl.get_precursor_coords(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_public_precursor(
  app: AppHandle,
  req: GetPublicPrecursorRequest,
) -> Result<PublicPrecursor> {
  app
    .client(async |cl| cl.get_public_precursor(req).await)
    .await?
    .map_err(Into::into)
}
