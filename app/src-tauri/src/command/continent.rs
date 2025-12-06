// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::continent::{Coord, PublicField};
use nil_payload::continent::{GetPublicFieldRequest, GetPublicFieldsRequest};
use tauri::AppHandle;

#[tauri::command]
pub async fn get_continent_size(app: AppHandle) -> Result<usize> {
  app
    .client(async |cl| cl.get_continent_size().await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_field(app: AppHandle, req: GetPublicFieldRequest) -> Result<PublicField> {
  app
    .client(async |cl| cl.get_field(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_fields(
  app: AppHandle,
  req: GetPublicFieldsRequest,
) -> Result<Vec<(Coord, PublicField)>> {
  if req.coords.is_empty() {
    return Ok(Vec::new());
  }

  app
    .client(async |cl| cl.get_fields(req).await)
    .await?
    .map_err(Into::into)
}
