// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use nil_client::nsr::{self, NsrScript};

#[tauri::command]
pub async fn fetch_nsr_readme(id: String) -> Result<String> {
  nsr::readme(&id).await.map_err(Into::into)
}

#[tauri::command]
pub async fn fetch_nsr_registry() -> Result<Vec<NsrScript>> {
  nsr::registry().await.map_err(Into::into)
}

#[tauri::command]
pub async fn fetch_nsr_script(id: String) -> Result<String> {
  nsr::script(&id).await.map_err(Into::into)
}
