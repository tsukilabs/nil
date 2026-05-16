// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::world::config::Locale;
use nil_server_types::auth::Token;
use serde::{Deserialize, Serialize};

#[cfg(feature = "typescript")]
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[derive_const(Default)]
#[serde(default, rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
#[cfg_attr(feature = "typescript", ts(rename = "app_Settings"))]
pub struct Settings {
  pub academy: AcademySettings,
  pub appearance: AppearanceSettings,
  pub auth: AuthSettings,
  pub general: GeneralSettings,
  pub prefecture: PrefectureSettings,
  pub stable: StableSettings,
  pub workshop: WorkshopSettings,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
#[cfg_attr(feature = "typescript", ts(rename = "app_AcademySettings"))]
pub struct AcademySettings {
  pub hide_unmet: bool,
}

impl const Default for AcademySettings {
  fn default() -> Self {
    Self { hide_unmet: true }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[derive_const(Default)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
#[cfg_attr(feature = "typescript", ts(rename = "app_AppearanceSettings"))]
pub struct AppearanceSettings {
  pub color_mode: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[derive_const(Default)]
#[serde(default, rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
#[cfg_attr(feature = "typescript", ts(rename = "app_AuthSettings"))]
pub struct AuthSettings {
  pub token: Option<Token>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
#[cfg_attr(feature = "typescript", ts(rename = "app_GeneralSettings"))]
pub struct GeneralSettings {
  pub auto_update: bool,
  pub hide_on_close: bool,
  pub locale: Locale,
}

impl const Default for GeneralSettings {
  #[cfg_attr(not(windows), allow(clippy::derivable_impls))]
  fn default() -> Self {
    Self {
      auto_update: cfg!(windows),
      hide_on_close: false,
      locale: Locale::default(),
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
#[cfg_attr(feature = "typescript", ts(rename = "app_PrefectureSettings"))]
pub struct PrefectureSettings {
  pub hide_maxed: bool,
  pub hide_unmet: bool,
}

impl const Default for PrefectureSettings {
  fn default() -> Self {
    Self { hide_maxed: false, hide_unmet: true }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
#[cfg_attr(feature = "typescript", ts(rename = "app_StableSettings"))]
pub struct StableSettings {
  pub hide_unmet: bool,
}

impl const Default for StableSettings {
  fn default() -> Self {
    Self { hide_unmet: true }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
#[cfg_attr(feature = "typescript", ts(rename = "app_WorkshopSettings"))]
pub struct WorkshopSettings {
  pub hide_unmet: bool,
}

impl const Default for WorkshopSettings {
  fn default() -> Self {
    Self { hide_unmet: true }
  }
}
