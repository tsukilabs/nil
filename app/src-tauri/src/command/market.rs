// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_payload::request::market::*;
use nil_payload::response::market::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn buy_resources(app: AppHandle, req: BuyResourcesRequest) -> Result<()> {
  app
    .client(async |cl| cl.buy_resources(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_market(app: AppHandle, req: GetMarketRequest) -> Result<GetMarketResponse> {
  app
    .client(async |cl| cl.get_market(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_market_fee(
  app: AppHandle,
  req: GetMarketFeeRequest,
) -> Result<GetMarketFeeResponse> {
  app
    .client(async |cl| cl.get_market_fee(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn sell_resources(app: AppHandle, req: SellResourcesRequest) -> Result<()> {
  app
    .client(async |cl| cl.sell_resources(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn send_resources(app: AppHandle, req: SendResourcesRequest) -> Result<()> {
  app
    .client(async |cl| cl.send_resources(req).await)
    .await
    .map_err(Into::into)
}
