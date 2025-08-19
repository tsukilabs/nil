// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::city::{City, PublicCity};
use nil_core::continent::Coord;
use nil_core::ranking::Score;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_city(app: AppHandle, coord: Coord) -> Result<City> {
  app
    .client(async |cl| cl.get_city(coord).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_city_score(app: AppHandle, coord: Coord) -> Result<Score> {
  app
    .client(async |cl| cl.get_city_score(coord).await)
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
