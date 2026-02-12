// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use itertools::Itertools;
use nil_core::continent::{ContinentSize, Coord, Distance, PublicField};
use nil_payload::continent::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_bulk_distance(origin: Coord, destinations: Vec<Coord>) -> Vec<(Coord, Distance)> {
  destinations
    .into_iter()
    .map(|destination| (destination, origin.distance(destination)))
    .collect()
}

#[tauri::command]
pub async fn get_continent_size(
  app: AppHandle,
  req: GetContinentSizeRequest,
) -> Result<ContinentSize> {
  app
    .client(async |cl| cl.get_continent_size(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_distance(origin: Coord, destination: Coord) -> Distance {
  origin.distance(destination)
}

#[tauri::command]
pub async fn get_public_field(app: AppHandle, req: GetPublicFieldRequest) -> Result<PublicField> {
  app
    .client(async |cl| cl.get_public_field(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_public_fields(
  app: AppHandle,
  mut req: GetPublicFieldsRequest,
) -> Result<Vec<(Coord, PublicField)>> {
  if req.coords.is_empty() {
    return Ok(Vec::new());
  }

  req.coords = req.coords.into_iter().unique().collect();

  app
    .client(async |cl| cl.get_public_fields(req).await)
    .await
    .map_err(Into::into)
}
