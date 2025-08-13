// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#[cfg(desktop)]
pub mod desktop;
#[cfg(mobile)]
pub mod mobile;

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

  define!("__DEBUG_ASSERTIONS__", cfg!(debug_assertions));
  define!("__DESKTOP__", cfg!(desktop));
  define!("__MOBILE__", cfg!(mobile));

  script
}
