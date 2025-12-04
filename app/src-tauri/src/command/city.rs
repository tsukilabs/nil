// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::city::{City, PublicCity};
use nil_core::ranking::Score;
use nil_payload::city::{
  FindPublicCityRequest,
  GetCityRequest,
  GetCityScoreRequest,
  GetPublicCityRequest,
  RenameCityRequest,
};
use tauri::AppHandle;

#[tauri::command]
pub async fn find_public_city(
  app: AppHandle,
  req: FindPublicCityRequest,
) -> Result<Option<PublicCity>> {
  app
    .client(async |cl| cl.find_public_city(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_city(app: AppHandle, req: GetCityRequest) -> Result<City> {
  app
    .client(async |cl| cl.get_city(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_city_score(app: AppHandle, req: GetCityScoreRequest) -> Result<Score> {
  app
    .client(async |cl| cl.get_city_score(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_public_city(app: AppHandle, req: GetPublicCityRequest) -> Result<PublicCity> {
  app
    .client(async |cl| cl.get_public_city(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn rename_city(app: AppHandle, req: RenameCityRequest) -> Result<()> {
  app
    .client(async |cl| cl.rename_city(req).await)
    .await?
    .map_err(Into::into)
}
