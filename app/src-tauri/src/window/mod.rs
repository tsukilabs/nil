// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#[cfg(desktop)]
pub mod desktop;
#[cfg(mobile)]
pub mod mobile;

use nil_core::continent::ContinentSize;
use nil_core::world::config::{BotAdvancedStartRatio, BotDensity};
use serde::Serialize;
use serde_json::json;
use tauri::WebviewUrl;

fn url() -> WebviewUrl {
  WebviewUrl::App("index.html".into())
}

fn script() -> String {
  let mut script = String::new();
  macro_rules! define {
    ($name:literal, $value:expr) => {{
      let name = $name;
      let value = json!($value);
      let snippet = format! {"
        Object.defineProperty(window, '{name}', {{
          configurable: false,
          enumerable: true,
          writable: false,
          value: {value},
        }});
      "};

      script.push_str(&snippet);
    }};
  }

  define!("__CONSTS__", Constants::default());
  define!("__DEBUG_ASSERTIONS__", cfg!(debug_assertions));
  define!("__DESKTOP__", cfg!(desktop));
  define!("__MOBILE__", cfg!(mobile));

  script
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Constants {
  i8_min: i8,
  i8_max: i8,
  i16_min: i16,
  i16_max: i16,
  u8_max: u8,
  u16_max: u16,
  u32_max: u32,

  bot_density_min: BotDensity,
  bot_density_max: BotDensity,
  bot_advanced_start_ratio_min: BotAdvancedStartRatio,
  bot_advanced_start_ratio_max: BotAdvancedStartRatio,
  continent_size_min: ContinentSize,
  continent_size_max: ContinentSize,
}

impl Default for Constants {
  fn default() -> Self {
    Self {
      i8_min: i8::MIN,
      i8_max: i8::MAX,
      i16_min: i16::MIN,
      i16_max: i16::MAX,
      u8_max: u8::MAX,
      u16_max: u16::MAX,
      u32_max: u32::MAX,

      bot_density_min: BotDensity::MIN,
      bot_density_max: BotDensity::MAX,
      bot_advanced_start_ratio_min: BotAdvancedStartRatio::MIN,
      bot_advanced_start_ratio_max: BotAdvancedStartRatio::MAX,
      continent_size_min: ContinentSize::MIN,
      continent_size_max: ContinentSize::MAX,
    }
  }
}
