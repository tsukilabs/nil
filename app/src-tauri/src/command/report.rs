// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use itertools::Itertools;
use nil_core::report::ReportKind;
use nil_payload::report::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_report(app: AppHandle, req: GetReportRequest) -> Result<ReportKind> {
  app
    .client(async |cl| cl.get_report(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_reports(app: AppHandle, mut req: GetReportsRequest) -> Result<Vec<ReportKind>> {
  if req.ids.is_empty() {
    return Ok(Vec::new());
  }

  req.ids = req.ids.into_iter().unique().collect();

  app
    .client(async |cl| cl.get_reports(req).await)
    .await
    .map_err(Into::into)
}
