// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_payload::request::report::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn forward_report(app: AppHandle, req: ForwardReportRequest) -> Result<()> {
  app
    .client(async |cl| cl.forward_report(req).await)
    .await
    .map_err(Into::into)
}
