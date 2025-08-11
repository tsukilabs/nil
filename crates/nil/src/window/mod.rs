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
  let debug = json!(cfg!(debug_assertions));
  format! {"
    Object.defineProperty(window, '__DEBUG_ASSERTIONS__', {{
      configurable: false,
      enumerable: true,
      writable: false,
      value: {debug},
    }});
  "}
}
