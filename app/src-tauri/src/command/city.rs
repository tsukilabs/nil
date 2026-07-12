// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_payload::request::city::*;
use nil_payload::response::city::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_cities(app: AppHandle, req: GetCitiesRequest) -> Result<GetCitiesResponse> {
  app
    .client(async |cl| cl.get_cities(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_city(app: AppHandle, req: GetCityRequest) -> Result<GetCityResponse> {
  app
    .client(async |cl| cl.get_city(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_city_score(
  app: AppHandle,
  req: GetCityScoreRequest,
) -> Result<GetCityScoreResponse> {
  app
    .client(async |cl| cl.get_city_score(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_public_cities(
  app: AppHandle,
  req: GetPublicCitiesRequest,
) -> Result<GetPublicCitiesResponse> {
  app
    .client(async |cl| cl.get_public_cities(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_public_city(
  app: AppHandle,
  req: GetPublicCityRequest,
) -> Result<GetPublicCityResponse> {
  app
    .client(async |cl| cl.get_public_city(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn rename_city(app: AppHandle, req: RenameCityRequest) -> Result<()> {
  app
    .client(async |cl| cl.rename_city(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn search_city(app: AppHandle, req: SearchCityRequest) -> Result<SearchCityResponse> {
  app
    .client(async |cl| cl.search_city(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn search_public_city(
  app: AppHandle,
  req: SearchPublicCityRequest,
) -> Result<SearchPublicCityResponse> {
  app
    .client(async |cl| cl.search_public_city(req).await)
    .await
    .map_err(Into::into)
}
