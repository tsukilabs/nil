// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_payload::user::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn create_user(app: AppHandle, req: CreateUserRequest) -> Result<()> {
  app
    .client(async |cl| cl.create_user(req).await)
    .await
    .map_err(Into::into)
}
