// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use itertools::Itertools;
use nil_payload::request::report::*;
use nil_payload::response::report::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn forward_report(app: AppHandle, req: ForwardReportRequest) -> Result<()> {
  app
    .client(async |cl| cl.forward_report(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_report(app: AppHandle, req: GetReportRequest) -> Result<GetReportResponse> {
  app
    .client(async |cl| cl.get_report(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_reports(app: AppHandle, mut req: GetReportsRequest) -> Result<GetReportsResponse> {
  if req.ids.is_empty() {
    return Ok(GetReportsResponse(Vec::new()));
  }

  req.ids = req.ids.into_iter().unique().collect();

  app
    .client(async |cl| cl.get_reports(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn remove_report(app: AppHandle, req: RemoveReportRequest) -> Result<()> {
  app
    .client(async |cl| cl.remove_report(req).await)
    .await
    .map_err(Into::into)
}
