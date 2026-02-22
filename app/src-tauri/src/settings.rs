// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::world::config::Locale;
use nil_server_types::Token;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
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
pub struct AcademySettings {
  pub hide_unmet: bool,
}

impl Default for AcademySettings {
  fn default() -> Self {
    Self { hide_unmet: true }
  }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppearanceSettings {
  pub color_mode: String,
  pub theme: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AuthSettings {
  pub token: Option<Token>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralSettings {
  pub auto_update: bool,
  pub hide_on_close: bool,
  pub locale: Locale,
}

impl Default for GeneralSettings {
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
pub struct PrefectureSettings {
  pub hide_maxed: bool,
  pub hide_unmet: bool,
}

impl Default for PrefectureSettings {
  fn default() -> Self {
    Self { hide_maxed: false, hide_unmet: true }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StableSettings {
  pub hide_unmet: bool,
}

impl Default for StableSettings {
  fn default() -> Self {
    Self { hide_unmet: true }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkshopSettings {
  pub hide_unmet: bool,
}

impl Default for WorkshopSettings {
  fn default() -> Self {
    Self { hide_unmet: true }
  }
}
