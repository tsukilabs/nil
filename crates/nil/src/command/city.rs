// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::city::{City, PublicCity};
use nil_core::continent::Coord;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_city(app: AppHandle, coord: Coord) -> Result<City> {
  app
    .client(async |cl| cl.get_city(coord).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_public_cities(app: AppHandle) -> Result<Vec<PublicCity>> {
  app
    .client(async |cl| cl.get_public_cities().await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_public_cities_by(app: AppHandle, coords: Vec<Coord>) -> Result<Vec<PublicCity>> {
  if coords.is_empty() {
    return Ok(Vec::new());
  }

  app
    .client(async |cl| cl.get_public_cities_by(coords).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_public_city(app: AppHandle, coord: Coord) -> Result<PublicCity> {
  app
    .client(async |cl| cl.get_public_city(coord).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn rename_city(app: AppHandle, coord: Coord, name: String) -> Result<()> {
  app
    .client(async |cl| cl.rename_city(coord, &name).await)
    .await?
    .map_err(Into::into)
}
